use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::encode_png,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn dinosaur(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("dinosaur/0.png")?;
    let mut surface = new_surface(frame.dimensions());
    let canvas = surface.canvas();
    let image = images[0].image.resize_fit((680, 578), Fit::Cover);
    canvas.draw_image(&image, (294, 369), None);
    canvas.draw_image(&frame, (0, 0), None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "dinosaur",
    dinosaur,
    min_images = 1,
    max_images = 1,
    keywords = &["恐龙", "小恐龙"],
    date_created = local_date(2023, 1, 6),
    date_modified = local_date(2023, 2, 14),
);
