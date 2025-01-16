use skia_safe::IRect;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    shortcut,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn wakeup(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let frame = load_image("wakeup/0.jpg")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(310, 270, 460, 380),
        text,
        50.0,
        90.0,
        None,
    )?;
    canvas
        .draw_text_area_auto_font_size(
            IRect::from_ltrb(50, 610, 670, 720),
            &format!("{text}起来了"),
            70.0,
            110.0,
            None,
        )
        .map_err(|_| Error::TextOverLength(text.to_string()))?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "wakeup",
    wakeup,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["好"],
    keywords = &["好起来了"],
    shortcuts = &[shortcut!(
        r"(?P<text>\S{1,4})\s+起来了",
        texts = &["{text}"],
        humanized = "xx 起来了",
    )],
    date_created = local_date(2022, 6, 12),
    date_modified = local_date(2023, 2, 14),
);
