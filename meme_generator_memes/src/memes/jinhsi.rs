use rand::RngExt;
use skia_safe::IRect;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::number_option, register_meme, tags::MemeTags};

number_option!(Number, 1, 13);

fn jinhsi(_: Vec<InputImage>, texts: Vec<String>, options: Number) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let num = options.number.unwrap_or(rand::rng().random_range(1..=13));

    let frame = load_image(format!("jinhsi/{:02}.png", num))?;
    let paddings = [55, 43, 50, 36, 40, 33, 36, 38, 33, 46, 26, 33, 28];
    let padding = paddings[(num - 1) as usize];

    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(0, frame.height() - padding, frame.width(), frame.height()),
        text,
        15.0,
        50.0,
        None,
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "jinhsi",
    jinhsi,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["汐汐"],
    keywords = &["汐汐", "今汐"],
    tags = MemeTags::jinhsi(),
    date_created = local_date(2024, 12, 7),
    date_modified = local_date(2024, 12, 7),
);
