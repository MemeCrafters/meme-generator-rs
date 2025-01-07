use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn your_new_years_eve(
    images: Vec<NamedImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("your_new_years_eve/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let img = images[0].resize_fit((586, 430), Fit::Contain);
        canvas.draw_image(&img, (0, 650), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "your_new_years_eve",
    your_new_years_eve,
    min_images = 1,
    max_images = 1,
    keywords = &["你的跨年"],
    date_created = local_date(2024, 12, 31),
    date_modified = local_date(2024, 12, 31),
);
