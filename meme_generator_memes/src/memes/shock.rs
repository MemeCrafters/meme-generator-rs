use rand::Rng;
use skia_safe::{Data, Image, Paint, RuntimeEffect};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{default_sampling_options, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn shock(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let sksl_code = r#"
        uniform shader image;
        uniform float2 direction;
        uniform float steps;

        half4 main(float2 coord) {
            half4 color = half4(0.0);
            float total_weight = 0.0;

            for (float i = -50; i <= 50; i++) {
                if (abs(i) > steps) {
                    continue;
                }
                float weight = 1.0 - abs(i) / steps;
                color += image.eval(coord + direction * i) * weight;
                total_weight += weight;
            }
            return color / total_weight;
        }
    "#;
    let effect = RuntimeEffect::make_for_shader(sksl_code, None).unwrap();

    let func = |_: usize, images: Vec<Image>| {
        let img = images[0].square().resize_exact((300, 300));
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(-90..=90);
        let angle = (angle as f32).to_radians();
        let direction = (angle.cos(), angle.sin());
        let steps = rng.gen_range(0..=50) as f32;
        let rotate = rng.gen_range(-20..=20) as f32;

        let mut values = Vec::new();
        for uniform in effect.uniforms() {
            match uniform.name() {
                "direction" => {
                    values.extend(direction.0.to_le_bytes());
                    values.extend(direction.1.to_le_bytes());
                }
                "steps" => values.extend(steps.to_le_bytes()),
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
        Ok(surface.image_snapshot().rotate_crop(rotate))
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 20,
            duration: 0.01,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "shock",
    shock,
    min_images = 1,
    max_images = 1,
    keywords = &["震惊"],
    date_created = local_date(2022, 3, 12),
    date_modified = local_date(2023, 2, 14),
);
