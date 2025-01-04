use skia_safe::{Color, IRect};

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

fn meteor(_: Vec<NamedImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let frame = load_image("meteor/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(220, 230, 920, 315),
        text,
        20.0,
        80.0,
        text_params!(paint = new_paint(Color::WHITE)),
    )?;
    encode_png(surface.image_snapshot())
}

register_meme!(
    "meteor",
    meteor,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["我要对象"],
    keywords = &["流星"],
    date_created = local_date(2022, 10, 21),
    date_modified = local_date(2023, 2, 14),
);
