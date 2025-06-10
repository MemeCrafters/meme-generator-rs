use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::encode_png,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn back_to_work(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("back_to_work/0.png")?;
    let mut surface = new_surface(frame.dimensions());
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    let image = images[0]
        .image
        .resize_fit((220, 310), Fit::Cover)
        .rotate(-25.0);
    canvas.draw_image(&image, (56, 32), None);
    canvas.draw_image(&frame, (0, 0), None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "back_to_work",
    back_to_work,
    min_images = 1,
    max_images = 1,
    keywords = &["继续干活", "打工人"],
    date_created = local_date(2022, 3, 10),
    date_modified = local_date(2023, 2, 14),
);
