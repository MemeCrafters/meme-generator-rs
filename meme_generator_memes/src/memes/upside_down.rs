use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

const DEFAULT_TEXT: &str = "我看你们是反了！";

fn upside_down(
    images: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };

    let img_w = 500;
    let text_h = 80;
    let mut text_surface = new_surface((img_w, text_h));
    let canvas = text_surface.canvas();
    canvas.draw_text_area_auto_font_size(
        skia_safe::IRect::from_ltrb(20, 0, img_w - 20, text_h),
        text,
        30.0,
        55.0,
        None,
    )?;
    let text_image = text_surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let img = images[0].resize_width(img_w).rotate(180.0);
        let img_h = img.height();
        let mut surface = new_surface((img_w, img_h + text_h));
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        canvas.draw_image(&text_image, (0, 0), None);
        canvas.draw_image(&img, (0, text_h), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "upside_down",
    upside_down,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["反了"],
    date_created = local_date(2024, 10, 12),
    date_modified = local_date(2024, 10, 12),
);
