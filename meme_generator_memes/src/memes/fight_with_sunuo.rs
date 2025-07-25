use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::encode_png,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn fight_with_sunuo(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("fight_with_sunuo/0.png")?;
    let mut surface = new_surface(frame.dimensions());
    let canvas = surface.canvas();
    let image = images[0]
        .image
        .resize_fit((565, 1630), Fit::Cover)
        .grayscale();
    canvas.draw_image(&image, (0, 245), None);
    canvas.draw_image(&frame, (0, 0), None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "fight_with_sunuo",
    fight_with_sunuo,
    min_images = 1,
    max_images = 1,
    keywords = &["我打宿傩", "我打宿傩吗"],
    tags = MemeTags::sukuna(),
    date_created = local_date(2024, 4, 3),
    date_modified = local_date(2024, 5, 25),
);
