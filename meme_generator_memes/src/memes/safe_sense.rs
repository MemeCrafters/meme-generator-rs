use skia_safe::{IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

const DEFAULT_TEXT: &str = "你给我的安全感\n远不及他的万分之一";

fn safe_sense(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };
    let frame = load_image("safe_sense/0.png")?;

    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(30, 0, 400, 130),
        text,
        25.0,
        50.0,
        None,
    )?;
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let img = images[0].resize_fit((215, 343), Fit::Cover);
        canvas.draw_image(&img, (215, 135), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "safe_sense",
    safe_sense,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 0,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["安全感"],
    date_created = local_date(2022, 3, 14),
    date_modified = local_date(2023, 2, 14),
);
