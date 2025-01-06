use skia_safe::IRect;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{color_from_hex_code, load_image, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn raise_sign(_: Vec<NamedImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let mut text_surface = new_surface((360, 260));
    let canvas = text_surface.canvas();
    let padding = 10;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(padding, padding, 350, 250),
        text,
        30.0,
        80.0,
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            paint = new_paint(color_from_hex_code("#51201b"))
        ),
    )?;
    let text_image = text_surface.image_snapshot();
    let text_image = text_image.perspective(&[(33, 0), (375, 120), (333, 387), (0, 258)]);

    let frame = load_image("raise_sign/0.jpg")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_image(&text_image, (285, 24), None);

    encode_png(surface.image_snapshot())
}

register_meme!(
    "raise_sign",
    raise_sign,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["大佬带带我"],
    keywords = &["举牌"],
    date_created = local_date(2022, 6, 12),
    date_modified = local_date(2023, 2, 14),
);
