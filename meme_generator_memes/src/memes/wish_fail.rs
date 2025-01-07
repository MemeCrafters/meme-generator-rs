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

fn wish_fail(_: Vec<NamedImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let frame = load_image("wish_fail/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(70, 305, 320, 380),
        text,
        20.0,
        80.0,
        None,
    )?;
    encode_png(surface.image_snapshot())
}

register_meme!(
    "wish_fail",
    wish_fail,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["我要对象"],
    keywords = &["许愿失败"],
    date_created = local_date(2022, 10, 21),
    date_modified = local_date(2023, 2, 14),
);
