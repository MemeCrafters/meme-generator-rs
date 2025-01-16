use skia_safe::{FontStyle, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

const DEFAULT_TEXT: &str = "你刚才说的话不是很礼貌！";

fn incivilization(
    images: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };

    let frame = load_image("incivilization/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(57, 42, 528, 117),
        text,
        20.0,
        50.0,
        text_params!(font_style = FontStyle::bold()),
    )?;
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image = images[0].circle().resize_exact((150, 150));
        let image = image.perspective(&[(0, 20), (154, 0), (164, 153), (22, 180)]);
        let image = image.brightness(0.8);
        canvas.draw_image(&image, (137, 151), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "incivilization",
    incivilization,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["不文明"],
    date_created = local_date(2022, 10, 12),
    date_modified = local_date(2023, 2, 14),
);
