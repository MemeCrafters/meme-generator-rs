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

fn slogan(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("slogan/0.jpg")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();

    let draw = |rect: IRect, text: &str| {
        canvas.draw_text_area_auto_font_size(rect, text, 15.0, 40.0, None)
    };

    draw(IRect::from_ltrb(10, 0, 294, 50), &texts[0])?;
    draw(IRect::from_ltrb(316, 0, 602, 50), &texts[1])?;
    draw(IRect::from_ltrb(10, 230, 294, 280), &texts[2])?;
    draw(IRect::from_ltrb(316, 230, 602, 280), &texts[3])?;
    draw(IRect::from_ltrb(10, 455, 294, 505), &texts[4])?;
    draw(IRect::from_ltrb(316, 455, 602, 505), &texts[5])?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "slogan",
    slogan,
    min_texts = 6,
    max_texts = 6,
    default_texts = &[
        "我们是谁？",
        "浙大人！",
        "到浙大来做什么？",
        "混！",
        "将来毕业后要做什么样的人？",
        "混混！",
    ],
    keywords = &["口号"],
    date_created = local_date(2022, 6, 12),
    date_modified = local_date(2023, 2, 14),
);
