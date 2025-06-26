use std::f32;

use skia_safe::{Data, Image, Paint, RuntimeEffect};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{default_sampling_options, load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn flick(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let sksl_code = r#"
        uniform shader image;
        uniform float width;
        uniform float height;
        uniform float offset;
        uniform float angle;

        half4 main(float2 coord) {
            float factor = 1.0 - coord.y / height;
            float dx = factor * offset;
            float a = angle * factor;
            float2 center = float2(width * 0.5, height * 0.5);
            coord -= center;
            float ca = cos(a);
            float sa = sin(a);
            float2 rotated = float2(
                coord.x * ca - coord.y * sa,
                coord.x * sa + coord.y * ca
            );
            coord = rotated + center;
            coord.x += dx;
            return image.eval(coord);
        }
    "#;
    let effect = RuntimeEffect::make_for_shader(sksl_code, None).unwrap();

    let func = |i: usize, images: Vec<Image>| {
        let img = images[0].square().resize_exact((240, 240));

        let frame = if i < 3 {
            img
        } else if i < 12 {
            let mut surface = img.to_surface();
            let canvas = surface.canvas();
            let hand = load_image(format!("flick/{}.png", i - 3))?;
            canvas.draw_image(&hand, (0, 0), None);
            surface.image_snapshot()
        } else {
            let width = img.width() as f32;
            let height = img.height() as f32;
            let amplitude = width / 2.0;
            let omega = 2.0 * f32::consts::PI / 5.0;
            let decay = 0.1;
            let max_angle = 0.5;
            let t = (i - 3) as f32;
            let damping = (-decay * t).exp();
            let offset = amplitude * (omega * t).sin() * damping;
            let angle = max_angle * (omega * t).sin() * damping;
            let mut values = Vec::new();
            for uniform in effect.uniforms() {
                match uniform.name() {
                    "width" => values.extend(width.to_le_bytes()),
                    "height" => values.extend(height.to_le_bytes()),
                    "offset" => values.extend(offset.to_le_bytes()),
                    "angle" => values.extend(angle.to_le_bytes()),
                    _ => {}
                }
            }
            let uniforms = Data::new_copy(&values);

            let image_shader = img
                .to_shader(None, default_sampling_options(), None)
                .unwrap();
            let shader = effect
                .make_shader(&uniforms, &[image_shader.into()], None)
                .unwrap();

            let mut surface = new_surface(img.dimensions());
            let canvas = surface.canvas();
            let mut paint = Paint::default();
            paint.set_shader(shader);
            canvas.draw_paint(&paint);
            if i == 12 {
                let hand = load_image(format!("flick/{}.png", i - 3))?;
                canvas.draw_image(&hand, (0, 0), None);
            }
            surface.image_snapshot()
        };
        Ok(frame)
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 30,
            duration: 0.05,
        },
        None,
    )
}

register_meme!(
    "flick",
    flick,
    min_images = 1,
    max_images = 1,
    keywords = &["弹", "脑瓜崩"],
    date_created = local_date(2025, 6, 22),
    date_modified = local_date(2025, 6, 22),
);
