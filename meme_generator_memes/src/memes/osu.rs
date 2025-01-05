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

fn osu(_: Vec<NamedImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let frame = load_image("osu/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(80, 80, 432, 432),
        text,
        80.0,
        192.0,
        text_params!(
            paint = new_paint(Color::WHITE),
            font_families = &["Aller"],
            font_style = FontStyle::bold()
        ),
    )?;
    encode_png(surface.image_snapshot())
}

register_meme!(
    "osu",
    osu,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["hso!"],
    keywords = &["osu"],
    date_created = local_date(2023, 7, 27),
    date_modified = local_date(2023, 7, 27),
);
