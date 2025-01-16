use skia_safe::{FontStyle, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    text_params,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

const DEFAULT_TEXT: &str = "偷学群友数理基础";

fn learn(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };
    let frame = load_image("learn/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(100, 1360, frame.width() - 100, 1730),
        text,
        200.0,
        350.0,
        text_params!(font_style = FontStyle::bold()),
    )?;
    let frame = surface.image_snapshot();

    let func = |imgs: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let img = imgs[0].resize_fit((1751, 1347), Fit::Cover);
        canvas.draw_image(&img, (1440, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "learn",
    learn,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["偷学"],
    date_created = local_date(2022, 12, 4),
    date_modified = local_date(2023, 2, 14),
);
