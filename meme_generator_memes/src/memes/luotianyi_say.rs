use skia_safe::IRect;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{color_from_hex_code, load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn luotianyi_say(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let frame = load_image("luotianyi_say/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(520, 0, frame.width() - 20, 220),
        text,
        40.0,
        140.0,
        text_params!(paint = new_paint(color_from_hex_code("#66CCFF"))),
    )?;
    encode_png(surface.image_snapshot())
}

register_meme!(
    "luotianyi_say",
    luotianyi_say,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["好想去海边啊～"],
    keywords = &["洛天依说", "天依说"],
    tags = MemeTags::luotianyi(),
    date_created = local_date(2025, 1, 7),
    date_modified = local_date(2025, 1, 7),
);
