use skia_safe::{IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

const DEFAULT_TEXT: &str = "笨蛋";

fn thermometer_gun(
    images: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if texts.is_empty() {
        DEFAULT_TEXT
    } else {
        &texts[0]
    };

    let mut text_surface = new_surface((200, 125));
    let canvas = text_surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(0, 0, 200, 125),
        text,
        15.0,
        60.0,
        text_params!(font_families = &["FZKaTong-M19S"]),
    )?;
    let text_frame = text_surface.image_snapshot();

    let img = &images[0].image;
    let img_w = img.width();
    let img_h = img.height();
    let (size, pos) = if img_w > img_h {
        ((img_h, img_h), (img_w - img_h, 0))
    } else {
        ((img_w, img_w), (0, img_h - img_w))
    };

    let frame = load_image("thermometer_gun/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_image(&text_frame, (555, 240), None);
    let frame = surface.image_snapshot();
    let frame = frame.resize_exact(size);

    let func = |images: Vec<Image>| {
        let img = &images[0];
        let mut surface = img.to_surface();
        let canvas = surface.canvas();
        canvas.draw_image(&frame, pos, None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "thermometer_gun",
    thermometer_gun,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["体温枪"],
    date_created = local_date(2024, 9, 3),
    date_modified = local_date(2024, 9, 3),
);
