use skia_safe::IRect;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn ascension(_: Vec<NamedImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = format!(
        "你原本应该要去地狱的，但因为你生前{}，我们就当作你已经服完刑期了",
        texts[0]
    );

    let frame = load_image("ascension/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas
        .draw_text_area_auto_font_size(IRect::from_ltrb(40, 30, 482, 135), text, 20.0, 50.0, None)
        .map_err(|_| Error::TextOverLength(texts[0].to_string()))?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "ascension",
    ascension,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["学的是机械"],
    keywords = &["升天"],
    date_created = local_date(2022, 10, 17),
    date_modified = local_date(2023, 2, 14),
);
