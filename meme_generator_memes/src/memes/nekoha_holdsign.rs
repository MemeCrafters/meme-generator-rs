use skia_safe::{Color, IRect};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn nekoha_holdsign(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let frame = load_image("nekoha_holdsign/0.jpg")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(210, 520, 570, 765),
        text,
        25.0,
        65.0,
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            paint = new_paint(Color::from_rgb(72, 110, 173))
        ),
    )?;
    encode_png(surface.image_snapshot())
}

register_meme!(
    "nekoha_holdsign",
    nekoha_holdsign,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["V我50"],
    keywords = &["猫羽雫举牌", "猫猫举牌"],
    tags = MemeTags::nekoha(),
    date_created = local_date(2023, 3, 30),
    date_modified = local_date(2023, 3, 30),
);
