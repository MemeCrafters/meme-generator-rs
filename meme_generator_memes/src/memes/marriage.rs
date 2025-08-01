use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::encode_png,
    image::{Fit, ImageExt},
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn marriage(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let left = load_image("marriage/0.png")?;
    let right = load_image("marriage/1.png")?;

    let img = images[0].image.resize_bound((1500, 1080), Fit::Contain);
    let mut surface = img.to_surface();
    let canvas = surface.canvas();
    canvas.draw_image(&left, (0, 0), None);
    canvas.draw_image(&right, (img.width() - right.width(), 0), None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "marriage",
    marriage,
    min_images = 1,
    max_images = 1,
    keywords = &["结婚申请", "结婚登记"],
    date_created = local_date(2022, 5, 31),
    date_modified = local_date(2023, 2, 14),
);
