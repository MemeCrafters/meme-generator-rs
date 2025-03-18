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

const DEFAULT_TEXT: &str = "兄弟，回南了";

fn fogging(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };

    let img = &images[0].image;
    let img_w = img.width().min(500);
    let img_h = img.height() * img_w / img.width();
    let mask = load_image("fogging/0.png")?;
    let mask = mask.resize_fit((img_w, img_h), Fit::Cover);

    let mut surface = mask.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(10, 10, mask.width() - 10, 80),
        text,
        20.0,
        40.0,
        None,
    )?;
    let mask = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let img = images[0].resize_exact((img_w, img_h));
        let img = img.gaussian_blur(10.0);
        let mut surface = img.to_surface();
        let canvas = surface.canvas();
        canvas.draw_image(&mask, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "fogging",
    fogging,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["回南天", "水雾"],
    default_texts = &[DEFAULT_TEXT],
    date_created = local_date(2025, 3, 16),
    date_modified = local_date(2025, 3, 16),
);
