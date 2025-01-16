use skia_safe::{Color, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    text_params,
    tools::{load_image, local_date, new_paint, new_stroke_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

const DEFAULT_TEXT: &str = "这是十二生肖中的鸡";

fn this_chicken(
    images: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };

    let frame = load_image("this_chicken/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(0, 900, 1440, 1080),
        text,
        40.0,
        80.0,
        text_params!(
            paint = new_paint(Color::WHITE),
            stroke_paint = new_stroke_paint(Color::BLACK, 10.0)
        ),
    )?;
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let img = images[0].resize_fit((640, 640), Fit::Cover);
        let img = img.perspective(&[(507, 0), (940, 351), (383, 625), (0, 256)]);
        canvas.draw_image(&img, (201, 201), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "this_chicken",
    this_chicken,
    min_images = 1,
    max_images = 1,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["这是鸡", "🐔"],
    date_created = local_date(2023, 11, 12),
    date_modified = local_date(2024, 1, 18),
);
