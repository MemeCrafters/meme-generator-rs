use skia_safe::IRect;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn run(_: Vec<NamedImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let mut text_surface = new_surface((122, 53));
    let canvas = text_surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(0, 0, 122, 53),
        text,
        10.0,
        50.0,
        None,
    )?;
    let text_image = text_surface.image_snapshot().rotate(-7.0);

    let frame = load_image("run/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_image(&text_image, (200, 195), None);

    encode_png(surface.image_snapshot())
}

register_meme!(
    "run",
    run,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["快跑"],
    keywords = &["快跑"],
    date_created = local_date(2022, 10, 17),
    date_modified = local_date(2023, 2, 14),
);
