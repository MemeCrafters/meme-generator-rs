use skia_safe::{Color, IRect, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::{Fit, ImageExt},
    text_params,
    tools::{local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

const DEFAULT_TEXT: &str = "如何提高社交质量 : \n远离以下头像的人";

fn keep_away(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };

    let mut surface = new_surface((400, 290));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    canvas
        .draw_text_area_auto_font_size(
            IRect::from_ltrb(10, 10, 390, 80),
            text,
            20.0,
            40.0,
            text_params!(text_align = TextAlign::Left),
        )
        .unwrap();

    let num_per_user = 8 / images.len();
    let total_images = images.len();
    let mut count = 0;

    let trans = |image: &Image, n: usize| -> Image {
        let img = image.square().resize_fit((100, 100), Fit::Cover);
        if n < 4 {
            img.rotate(n as f32 * 90.0)
        } else {
            img.flip_horizontal().rotate((n - 4) as f32 * 90.0)
        }
    };

    let mut paste = |image: Image| {
        let y = if count < 4 { 90 } else { 190 };
        canvas.draw_image(&image, (count % 4 * 100, y), None);
        count += 1;
    };

    for image in images.iter() {
        for n in 0..num_per_user {
            paste(trans(&image.image, n));
        }
    }

    let num_left = 8 - num_per_user * total_images;
    for n in 0..num_left {
        paste(trans(&images[images.len() - 1].image, n + num_per_user));
    }

    encode_png(surface.image_snapshot())
}

register_meme!(
    "keep_away",
    keep_away,
    min_images = 1,
    max_images = 8,
    min_texts = 0,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["远离"],
    date_created = local_date(2022, 5, 31),
    date_modified = local_date(2023, 2, 14),
);
