use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::encode_png,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn rip_angrily(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("rip_angrily/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    let img = images[0].image.square().resize_exact((105, 105));
    canvas.draw_image(&img.rotate(24.0), (18, 170), None);
    canvas.draw_image(&img.rotate(-24.0), (163, 65), None);
    canvas.draw_image(&frame, (0, 0), None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "rip_angrily",
    rip_angrily,
    min_images = 1,
    max_images = 1,
    keywords = &["怒撕"],
    date_created = local_date(2022, 10, 9),
    date_modified = local_date(2023, 2, 14),
);
