use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn walnut_pad(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("walnut_pad/0.png")?;

    let func = |images: Vec<Image>| {
        let img = images[0].resize_fit((540, 360), Fit::Cover);
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        canvas.draw_image(&img, (368, 65), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "walnut_pad",
    walnut_pad,
    min_images = 1,
    max_images = 1,
    keywords = &["胡桃平板"],
    tags = MemeTags::walnut(),
    date_created = local_date(2022, 8, 7),
    date_modified = local_date(2023, 2, 14),
);
