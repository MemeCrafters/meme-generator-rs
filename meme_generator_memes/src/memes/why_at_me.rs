use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::encode_png,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn why_at_me(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("why_at_me/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    let img = images[0]
        .image
        .square()
        .resize_exact((265, 265))
        .rotate_crop(-19.0);
    canvas.draw_image(&img, (42, 13), None);
    canvas.draw_image(&frame, (0, 0), None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "why_at_me",
    why_at_me,
    min_images = 1,
    max_images = 1,
    keywords = &["为什么@我"],
    tags = MemeTags::touhou(),
    date_created = local_date(2022, 4, 14),
    date_modified = local_date(2023, 5, 3),
);
