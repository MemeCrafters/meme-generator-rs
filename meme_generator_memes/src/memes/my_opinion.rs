use skia_safe::{FontStyle, IRect};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn my_opinion(_: Vec<NamedImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let frame = load_image("my_opinion/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(37, 660, 487, 1070),
        text,
        50.0,
        500.0,
        text_params!(font_style = FontStyle::bold()),
    )?;
    encode_png(surface.image_snapshot())
}

register_meme!(
    "my_opinion",
    my_opinion,
    min_texts = 1,
    max_texts = 1,
    keywords = &["我的意见如下", "我的意见是"],
    tags = MemeTags::touhou(),
    date_created = local_date(2024, 7, 14),
    date_modified = local_date(2024, 7, 14),
);
