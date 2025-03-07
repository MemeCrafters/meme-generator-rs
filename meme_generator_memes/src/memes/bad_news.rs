use skia_safe::{Color, IRect};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_stroke_paint},
};

use crate::{options::NoOptions, register_meme};

fn bad_news(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let frame = load_image("bad_news/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(50, 100, frame.width() - 50, frame.height() - 100),
        text,
        30.0,
        60.0,
        text_params!(stroke_paint = new_stroke_paint(Color::WHITE, 5.0)),
    )?;
    encode_png(surface.image_snapshot())
}

register_meme!(
    "bad_news",
    bad_news,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["喜报"],
    keywords = &["悲报"],
    date_created = local_date(2022, 10, 15),
    date_modified = local_date(2023, 2, 14),
);
