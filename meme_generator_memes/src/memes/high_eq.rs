use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    shortcut, text_params,
    tools::{load_image, local_date, new_paint, new_stroke_paint},
};

use crate::{options::NoOptions, register_meme};

fn high_eq(_: Vec<DecodedImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("high_eq/0.jpg")?;

    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    let draw = |pos: (i32, i32, i32, i32), text: &str| {
        canvas.draw_text_area_auto_font_size(
            skia_safe::IRect::from_ltrb(pos.0, pos.1, pos.2, pos.3),
            text,
            50.0,
            100.0,
            text_params!(
                paint = new_paint(Color::WHITE),
                stroke_paint = new_stroke_paint(Color::BLACK, 5.0)
            ),
        )
    };
    draw((40, 540, 602, 1140), &texts[0])?;
    draw((682, 540, 1244, 1140), &texts[1])?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "high_eq",
    high_eq,
    min_texts = 2,
    max_texts = 2,
    default_texts = &["高情商", "低情商"],
    keywords = &["高低情商", "低高情商"],
    shortcuts = &[
        shortcut!(
            r"低情商[\s:：]*(?P<low>\S+)\s*高情商[\s:：]*(?P<high>\S+)",
            texts = &["${low}", "${high}"],
            humanized = "低情商xx高情商xx",
        ),
        shortcut!(
            r"高情商[\s:：]*(?P<high>\S+)\s*低情商[\s:：]*(?P<low>\S+)",
            texts = &["${low}", "${high}"],
            humanized = "高情商xx低情商xx",
        ),
    ],
    date_created = local_date(2022, 6, 12),
    date_modified = local_date(2024, 8, 12),
);
