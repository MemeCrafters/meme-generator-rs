use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn sekaiichi_kawaii(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let img = &images[0].image;
    let img_w = img.width();
    let img_h = img.height();
    let (frame, size) = if (img_w as f32 / img_h as f32) > 1.155 {
        (load_image("sekaiichi_kawaii/0.png")?, (810, 416))
    } else {
        (load_image("sekaiichi_kawaii/1.png")?, (585, 810))
    };

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let img = images[0].resize_fit(size, Fit::Cover);
        canvas.draw_image(&img, (45, 45), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "sekaiichi_kawaii",
    sekaiichi_kawaii,
    min_images = 1,
    max_images = 1,
    keywords = &["世界第一可爱"],
    tags = MemeTags::kotone(),
    date_created = local_date(2024, 12, 4),
    date_modified = local_date(2024, 12, 4),
);
