use skia_safe::{Color, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    text_params,
    tools::{load_image, local_date, new_paint, new_stroke_paint, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

const DEFAULT_TEXT: &str = "所谓的男人啊，只要送他们这种东西就会很开心";

fn frieren_take(
    images: &mut Vec<DecodedImage>,
    texts: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };
    let frame = load_image("frieren_take/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(
            100,
            frame.height() - 120,
            frame.width() - 100,
            frame.height(),
        ),
        text,
        20.0,
        30.0,
        text_params!(
            paint = new_paint(Color::WHITE),
            stroke_paint = new_stroke_paint(Color::BLACK, 4.0)
        ),
    )?;
    let frame = surface.image_snapshot();

    let func = |images: &Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].resize_fit((102, 108), Fit::Cover);
        canvas.draw_image(&image, (130, 197), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "frieren_take",
    frieren_take,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["芙莉莲拿"],
    tags = MemeTags::frieren(),
    date_created = local_date(2024, 1, 18),
    date_modified = local_date(2024, 8, 9),
);
