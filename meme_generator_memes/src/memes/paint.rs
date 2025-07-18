use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::encode_png,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn paint(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("paint/0.png")?;
    let mut surface = new_surface(frame.dimensions());
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    let img = images[0]
        .image
        .resize_fit((117, 135), Fit::Cover)
        .rotate(-4.0);
    canvas.draw_image(&img, (95, 107), None);
    canvas.draw_image(&frame, (0, 0), None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "paint",
    paint,
    min_images = 1,
    max_images = 1,
    keywords = &["这像画吗"],
    date_created = local_date(2022, 3, 11),
    date_modified = local_date(2023, 2, 14),
);
