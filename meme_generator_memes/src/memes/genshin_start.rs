use skia_safe::{Color, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    text_params,
    tools::{load_image, local_date, new_paint, new_stroke_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

const DEFAULT_TEXT: &str = "原神，启动！";

fn genshin_start(
    images: Vec<DecodedImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };
    let frame = load_image("genshin_start/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(
            100,
            frame.height() - 150,
            frame.width() - 100,
            frame.height(),
        ),
        text,
        70.0,
        100.0,
        text_params!(
            paint = new_paint(Color::WHITE),
            stroke_paint = new_stroke_paint(Color::BLACK, 10.0)
        ),
    )?;
    let frame = surface.image_snapshot();

    let func = |images: &Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        let image = images[0].resize_fit((600, 330), Fit::Cover).perspective(&[
            (0, 116),
            (585, 0),
            (584, 319),
            (43, 385),
        ]);
        canvas.draw_image(&image, (412, 121), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "genshin_start",
    genshin_start,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["原神启动"],
    date_created = local_date(2023, 7, 1),
    date_modified = local_date(2023, 7, 1),
);
