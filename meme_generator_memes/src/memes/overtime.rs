use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::encode_png,
    image::{Fit, ImageExt},
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn overtime(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("overtime/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    let img = images[0]
        .image
        .resize_fit((250, 250), Fit::Cover)
        .rotate(25.0);
    canvas.draw_image(&img, (165, 220), None);
    canvas.draw_image(&frame, (0, 0), None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "overtime",
    overtime,
    min_images = 1,
    max_images = 1,
    keywords = &["加班"],
    date_created = local_date(2023, 1, 8),
    date_modified = local_date(2023, 2, 14),
);
