use skia_safe::{Color, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn stew(images: Vec<NamedImage>, texts: Vec<String>, args: NoOptions) -> Result<Vec<u8>, Error> {
    let name = &images[0].name;
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        &format!("生活不易,炖{name}出气")
    };

    let frame = load_image("stew/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(2, frame.height() - 30, frame.width() - 2, frame.height()),
        text,
        6.0,
        30.0,
        None,
    )?;
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].resize_fit((181, 154), Fit::Cover);
        canvas.draw_image(&image, (9, -2), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "stew",
    stew,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["炖"],
    date_created = local_date(2024, 1, 19),
    date_modified = local_date(2024, 1, 19),
);
