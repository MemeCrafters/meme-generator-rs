use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::encode_png,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn hold_tight(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("hold_tight/0.png")?;
    let mut surface = new_surface(frame.dimensions());
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    let image = images[0].image.resize_fit((159, 171), Fit::Cover);
    canvas.draw_image(&image, (113, 205), None);
    canvas.draw_image(&frame, (0, 0), None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "hold_tight",
    hold_tight,
    min_images = 1,
    max_images = 1,
    keywords = &["抱紧"],
    date_created = local_date(2022, 10, 1),
    date_modified = local_date(2023, 2, 14),
);
