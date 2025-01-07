use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn swirl_turn(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame_num = 40;

    let func = |i: usize, images: Vec<Image>| {
        let mut surface = new_surface((300, 300));
        let canvas = surface.canvas();
        canvas.clear(Color::TRANSPARENT);
        let img = images[0].circle().resize_exact((100, 100));
        let start_angle = i as f32 * 360.0 / frame_num as f32;
        let num = 24;
        for j in 0..num {
            let angle = start_angle + j as f32 * 360.0 / num as f32;
            let x = 150.0 + 75.0 * angle.to_radians().cos();
            let y = 150.0 + 75.0 * angle.to_radians().sin();
            canvas.draw_image(&img, (x.round() as i32 - 50, y.round() as i32 - 50), None);
        }
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num,
            duration: 0.02,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "swirl_turn",
    swirl_turn,
    min_images = 1,
    max_images = 1,
    keywords = &["回旋转", "旋风转"],
    date_created = local_date(2024, 5, 7),
    date_modified = local_date(2024, 5, 7),
);
