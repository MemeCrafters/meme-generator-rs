use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::encode_png,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn karyl_point(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("karyl_point/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    let image = images[0]
        .image
        .square()
        .resize_exact((200, 200))
        .rotate(-7.5);
    canvas.draw_image(&image, (87, 790), None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "karyl_point",
    karyl_point,
    min_images = 1,
    max_images = 1,
    keywords = &["凯露指"],
    tags = MemeTags::karyl(),
    date_created = local_date(2022, 11, 16),
    date_modified = local_date(2023, 2, 14),
);
