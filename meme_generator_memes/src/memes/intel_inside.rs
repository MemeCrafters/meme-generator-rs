use skia_safe::{Color, FontStyle, IRect};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme};

fn intel_inside(_: Vec<NamedImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];

    let frame = load_image("intel_inside/0.jpg")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(240, 340, 680, 580),
        text,
        80.0,
        180.0,
        text_params!(
            font_families = &["Neo Sans"],
            paint = new_paint(Color::WHITE),
            font_style = FontStyle::bold()
        ),
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "intel_inside",
    intel_inside,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["intel"],
    keywords = &["inside"],
    date_created = local_date(2024, 10, 29),
    date_modified = local_date(2024, 10, 29),
);
