use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::encode_png,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn sit_still(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("sit_still/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    let img = images[0].image.circle().resize_exact((150, 150));
    canvas.draw_image(&img, (268, 344), None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "sit_still",
    sit_still,
    min_images = 1,
    max_images = 1,
    keywords = &["坐得住"],
    date_created = local_date(2022, 12, 3),
    date_modified = local_date(2023, 2, 14),
);
