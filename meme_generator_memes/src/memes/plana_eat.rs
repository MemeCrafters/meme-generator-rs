use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::encode_png,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn plana_eat(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("plana_eat/0.png")?;
    let mut surface = new_surface(frame.dimensions());
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    let img = images[0].image.resize_fit((488, 488), Fit::Cover);
    canvas.draw_image(&img, (212, 535), None);
    canvas.draw_image(&frame, (0, 0), None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "plana_eat",
    plana_eat,
    min_images = 1,
    max_images = 1,
    keywords = &["普拉娜吃", "普拉娜舔"],
    tags = MemeTags::plana(),
    date_created = local_date(2024, 11, 21),
    date_modified = local_date(2024, 11, 21),
);
