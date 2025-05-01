use skia_safe::{IRect, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{color_from_hex_code, load_image, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn ayachi_holdsign(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let mut text_surface = new_surface((600, 350));
    let canvas = text_surface.canvas();
    let padding = 20;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(padding, padding, 580, 330),
        text,
        80.0,
        150.0,
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            text_align = TextAlign::Center,
            paint = new_paint(color_from_hex_code("#51201b"))
        ),
    )?;
    let text_image = text_surface.image_snapshot();
    let text_image = text_image.perspective(&[(0, 235), (523, 0), (659, 297), (170, 536)]);

    let frame = load_image("ayachi_holdsign/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_image(&text_image, (125, 307), None);

    encode_png(surface.image_snapshot())
}

register_meme!(
    "ayachi_holdsign",
    ayachi_holdsign,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["我控制不住自己啊"],
    keywords = &["宁宁举牌"],
    tags = MemeTags::ayachi(),
    date_created = local_date(2025, 4, 28),
    date_modified = local_date(2025, 4, 28),
);
