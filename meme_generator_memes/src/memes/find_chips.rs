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

fn find_chips(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("find_chips/0.jpg")?;

    let mut surface = frame.to_surface();
    let canvas = surface.canvas();

    let draw = |pos: (i32, i32, i32, i32), text: &str| {
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(pos.0, pos.1, pos.2, pos.3),
            text,
            12.0,
            30.0,
            None,
        )
    };

    draw((405, 54, 530, 130), &texts[0])?;
    draw((570, 62, 667, 160), &texts[1])?;
    draw((65, 400, 325, 463), &texts[2])?;
    draw((430, 400, 630, 470), &texts[3])?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "find_chips",
    find_chips,
    min_texts = 4,
    max_texts = 4,
    default_texts = &[
        "我们要飞向何方",
        "我打算待会去码头整点薯条",
        "我说的是归根结底，活着是为了什么",
        "为了待会去码头整点薯条",
    ],
    keywords = &["整点薯条"],
    date_created = local_date(2022, 10, 26),
    date_modified = local_date(2023, 2, 14),
);
