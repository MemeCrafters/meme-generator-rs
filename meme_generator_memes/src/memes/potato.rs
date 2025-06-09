use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::encode_png,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn potato(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("potato/0.png")?;
    let mut surface = new_surface(frame.dimensions());
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    let img = images[0]
        .image
        .square()
        .resize_exact((458, 458))
        .rotate_crop(5.0);
    canvas.draw_image(&img, (531, 15), None);
    canvas.draw_image(&frame, (0, 0), None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "potato",
    potato,
    min_images = 1,
    max_images = 1,
    keywords = &["土豆"],
    date_created = local_date(2023, 1, 19),
    date_modified = local_date(2023, 2, 14),
);
