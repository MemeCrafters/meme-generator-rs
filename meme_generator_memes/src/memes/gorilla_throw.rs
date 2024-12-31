use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::{make_gif_or_combined_gif, GifInfo},
    image::{Fit, ImageExt},
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn gorilla_throw(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let params = [
        (74, 125, 24, 8, 135),
        (84, 119, 24, 8, 130),
        (111, 103, 22, 15, 100),
        (122, 95, 22, 15, 90),
        (136, 89, 25, 17, 87),
        (142, 60, 49, 22, 40),
        (134, 49, 66, 30, 30),
        (134, 49, 66, 30, 30),
        (116, 35, 92, 38, 25),
        (78, 26, 167, 73, 5),
        (-30, 0, 300, 180, -5),
        (-120, -85, 400, 240, -16),
        (-160, -125, 500, 300, -20),
        (-180, -180, 600, 360, -23),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("gorilla_throw/{i:02}.png"))?;
        if i < 28 {
            return Ok(frame);
        }
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let (x, y, w, h, a) = params[i - 28];
        let image = images[0].resize_fit((w, h), Fit::Cover).rotate(a as f32);
        canvas.draw_image(&image, (x, y), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 42,
            duration: 0.04,
        },
        None,
    )
}

register_meme!(
    "gorilla_throw",
    gorilla_throw,
    min_images = 1,
    max_images = 1,
    keywords = &["猩猩扔"],
    date_created = local_date(2024, 11, 16),
    date_modified = local_date(2024, 11, 22),
);
