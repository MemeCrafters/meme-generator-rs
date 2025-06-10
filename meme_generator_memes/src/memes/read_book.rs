use skia_safe::{Color, IRect};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::{Fit, ImageExt},
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

const DEFAULT_TEXT: &str = "エロ本";

fn read_book(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };

    let mut text_surface = new_surface((775, 300));
    let canvas = text_surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(0, 50, 775, 250),
        text,
        50.0,
        150.0,
        text_params!(paint = new_paint(Color::WHITE)),
    )?;
    let text_image = text_surface.image_snapshot().rotate(88.0);

    let frame = load_image("read_book/0.png")?;
    let mut surface = new_surface(frame.dimensions());
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    let img = &images[0].image.resize_fit((1000, 1100), Fit::Cover);
    let img = img.perspective(&[(0, 108), (1092, 0), (1023, 1134), (29, 1134)]);
    canvas.draw_image(&img, (1138, 1172), None);
    canvas.draw_image(&frame, (0, 0), None);
    canvas.draw_image(&text_image, (826, 1496), None);

    encode_png(surface.image_snapshot())
}

register_meme!(
    "read_book",
    read_book,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["看书"],
    date_created = local_date(2022, 8, 22),
    date_modified = local_date(2023, 10, 25),
);
