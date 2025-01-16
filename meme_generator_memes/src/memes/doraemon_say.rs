use skia_safe::IRect;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn doraemon_say(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let frame = load_image("doraemon_say/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(228, 11, 340, 164),
        text,
        20.0,
        80.0,
        None,
    )?;
    encode_png(surface.image_snapshot())
}

register_meme!(
    "doraemon_say",
    doraemon_say,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["开银趴不喊我是吧"],
    keywords = &["哆啦A梦说"],
    date_created = local_date(2022, 11, 16),
    date_modified = local_date(2023, 2, 14),
);
