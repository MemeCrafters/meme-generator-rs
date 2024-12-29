use skia_safe::{IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn father_work(
    images: &mut Vec<DecodedImage>,
    texts: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("father_work/0.png")?;
    let text = &texts[0];

    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(
            195,
            frame.height() - 110,
            frame.width() - 10,
            frame.height() - 20,
        ),
        text,
        10.0,
        50.0,
        None,
    )?;
    let frame = surface.image_snapshot();

    let func = |images: &Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image = images[0].resize_fit((230, 120), Fit::Contain);
        canvas.draw_image(&image, (252, 142), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "father_work",
    father_work,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    keywords = &["我爸爸"],
    date_created = local_date(2024, 5, 12),
    date_modified = local_date(2024, 5, 16),
);
