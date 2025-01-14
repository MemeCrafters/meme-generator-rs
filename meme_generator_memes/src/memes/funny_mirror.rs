use core::f32;

use skia_safe::{Data, Image, Paint, RuntimeEffect, SamplingOptions};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    tools::{local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn funny_mirror(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let sksl_code = r#"
        uniform shader image;
        uniform float2 center;
        uniform float strength;
        uniform float radius;

        half4 main(float2 coord) {
            float2 offset = coord - center;
            float distance = length(offset);
            offset *= mix(1.0, smoothstep(0.0, radius / distance, distance / radius), strength);
            return image.eval(center + offset);
        }
    "#;
    let effect = RuntimeEffect::make_for_shader(sksl_code, None).unwrap();

    let func = |i: usize, images: Vec<Image>| {
        let img = &images[0];

        let strength = 0.5 * (f32::consts::PI / 20.0 * i as f32).sin();
        let x = img.width() as f32 * 0.5;
        let y = img.height() as f32 * 0.5;
        let radius = (x * x + y * y).sqrt();

        let mut values = Vec::new();
        for uniform in effect.uniforms() {
            match uniform.name() {
                "center" => {
                    values.extend(x.to_le_bytes());
                    values.extend(y.to_le_bytes());
                }
                "strength" => values.extend(strength.to_le_bytes()),
                "radius" => values.extend(radius.to_le_bytes()),
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

        let mut surface = new_surface(img.dimensions());
        let canvas = surface.canvas();
        let mut paint = Paint::default();
        paint.set_shader(shader);
        canvas.draw_paint(&paint);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 21,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "funny_mirror",
    funny_mirror,
    min_images = 1,
    max_images = 1,
    keywords = &["哈哈镜"],
    date_created = local_date(2022, 3, 13),
    date_modified = local_date(2025, 1, 14),
);
