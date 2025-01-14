use skia_safe::{Color, Data, Image, Paint, RuntimeEffect, SamplingOptions};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn lost_dog(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let k = 2;
    let w_ = 663 * k;
    let w = 540 * k;
    let h = 540 * k;
    let r = 466 * k;

    let sksl_code = r#"
        uniform shader image;
        uniform float w_;
        uniform float w;
        uniform float h;
        uniform float r;

        half4 main(float2 coord) {
            float dx = coord.x - w / 2;
            float dy = coord.y - h / 2;
            float theta = asin(abs(dx) / r);
            float x_ = w_ / 2 + dx / cos(theta);
            float y_ = h / 2 + dy / cos(theta);

            if (x_ >= 0 && x_ < w_ && y_ >= 0 && y_ < h){
                return image.eval(float2(x_, y_));
            }
            return half4(0);
        }
    "#;
    let effect = RuntimeEffect::make_for_shader(sksl_code, None).unwrap();
    let mut values = Vec::new();
    for uniform in effect.uniforms() {
        match uniform.name() {
            "w_" => values.extend((w_ as f32).to_le_bytes()),
            "w" => values.extend((w as f32).to_le_bytes()),
            "h" => values.extend((h as f32).to_le_bytes()),
            "r" => values.extend((r as f32).to_le_bytes()),
            _ => {}
        }
    }
    let uniforms = Data::new_copy(&values);

    let func = |images: Vec<Image>| {
        let img = images[0].resize_fit((w_, h), Fit::Cover);

        let image_shader = img
            .to_shader(None, SamplingOptions::default(), None)
            .unwrap();
        let shader = effect
            .make_shader(&uniforms, &[image_shader.into()], None)
            .unwrap();

        let mut surface = new_surface((w, h));
        let canvas = surface.canvas();
        let mut paint = Paint::default();
        paint.set_shader(shader);
        canvas.draw_paint(&paint);
        let img = surface.image_snapshot();

        let frame = load_image("lost_dog/0.png")?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let img = img.resize_exact((w / k, h / k));
        canvas.draw_image(&img, (295, 165), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "lost_dog",
    lost_dog,
    min_images = 1,
    max_images = 1,
    keywords = &["寻狗启事"],
    tags = MemeTags::ayaka(),
    date_created = local_date(2024, 1, 19),
    date_modified = local_date(2024, 1, 20),
);
