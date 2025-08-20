use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::encode_png,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn you_dont_get(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let img = images[0].image.resize_fit((142, 139), Fit::Cover);
    let frame = load_image("you_dont_get/0.png")?;
    let mut surface = new_surface(frame.dimensions());
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    canvas.draw_image(&img, (217, 181), None);
    canvas.draw_image(&frame, (0, 0), None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "you_dont_get",
    you_dont_get,
    min_images = 1,
    max_images = 1,
    keywords = &["你不懂啦"],
    tags = MemeTags::capoo(),
    date_created = local_date(2025, 5, 15),
    date_modified = local_date(2025, 5, 15),
);
