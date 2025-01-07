use skia_safe::{Color, FontStyle, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    text_params,
    tools::{load_image, local_date, new_paint, new_stroke_paint},
};

use crate::{options::NoOptions, register_meme};

const DEFAULT_TEXT: &str = "今年520";

fn what_he_wants(
    images: Vec<NamedImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let date = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };
    let text = format!("{}我会给你每个男人都最想要的东西···", date);
    let frame = load_image("what_he_wants/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(0, 514, 1024, 614),
        &text,
        20.0,
        80.0,
        text_params!(
            paint = new_paint(Color::BLACK),
            stroke_paint = new_stroke_paint(Color::WHITE, 10.0),
            font_style = FontStyle::bold()
        ),
    )?;
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let img = images[0].resize_fit((538, 538), Fit::Contain);
        canvas.draw_image(&img, (486, 616), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "what_he_wants",
    what_he_wants,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["最想要的东西"],
    date_created = local_date(2023, 5, 20),
    date_modified = local_date(2023, 5, 20),
);
