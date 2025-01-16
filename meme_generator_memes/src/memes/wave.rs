use core::f32;

use skia_safe::{Data, IRect, Image, Paint, RuntimeEffect, SamplingOptions};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn wave(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let img = &images[0].image;
    let img_w = img.width().max(360).min(720);
    let period = img_w as f32 / 6.0;
    let amp = img_w as f32 / 60.0;
    let frame_num = 8;

    let sksl_code = r#"
        uniform shader image;
        uniform float w;
        uniform float h;
        uniform float a;
        uniform float omega;
        uniform float phi;

        half4 main(float2 coord) {
            float x = coord.x;
            float y = coord.y;
            float dx = a * sin(omega * (x + phi)) * (h - y) / h;
            float dy = a * sin(omega * (y + phi)) * y / h;
            float x_ = x + dx;
            float y_ = y + dy;
            if (x_ >= 0 && x_ < w && y_ >= 0 && y_ < h){
                return image.eval(float2(x_, y_));
            }
            return image.eval(float2(x, y));
        }
    "#;
    let effect = RuntimeEffect::make_for_shader(sksl_code, None).unwrap();

    let func = |i: usize, images: Vec<Image>| {
        let img = images[0].resize_width(img_w);
        let img_h = img.height();

        let mut values = Vec::new();
        for uniform in effect.uniforms() {
            match uniform.name() {
                "w" => values.extend((img_w as f32).to_le_bytes()),
                "h" => values.extend((img_h as f32).to_le_bytes()),
                "a" => values.extend((amp as f32 / 2.0).to_le_bytes()),
                "omega" => values.extend((2.0 * f32::consts::PI / period).to_le_bytes()),
                "phi" => values.extend((i as f32 * period / frame_num as f32).to_le_bytes()),
                _ => {}
            }
        }
        let uniforms = Data::new_copy(&values);
        let image_shader = img
            .to_shader(None, SamplingOptions::default(), None)
            .unwrap();
        let shader = effect
            .make_shader(&uniforms, &[image_shader.into()], None)
            .unwrap();

        let frame_w = img_w - amp as i32;
        let frame_h = img_h - amp as i32;
        let mut surface = new_surface((frame_w, frame_h));
        let canvas = surface.canvas();
        let mut paint = Paint::default();
        paint.set_shader(shader);
        canvas.draw_irect(
            IRect::from_xywh((frame_w - img_w) / 2, (frame_h - img_h) / 2, img_w, img_h),
            &paint,
        );
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num,
            duration: 0.01,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "wave",
    wave,
    min_images = 1,
    max_images = 1,
    keywords = &["波纹"],
    date_created = local_date(2022, 10, 26),
    date_modified = local_date(2023, 2, 14),
);
