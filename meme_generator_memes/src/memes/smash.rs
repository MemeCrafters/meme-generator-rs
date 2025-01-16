use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn smash(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("smash/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let screen = images[0].resize_fit((800, 500), Fit::Cover);
        let screen = screen.perspective(&[(1, 237), (826, 1), (832, 508), (160, 732)]);
        canvas.draw_image(&screen, (-136, -81), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "smash",
    smash,
    min_images = 1,
    max_images = 1,
    keywords = &["砸"],
    date_created = local_date(2022, 11, 29),
    date_modified = local_date(2023, 2, 14),
);
