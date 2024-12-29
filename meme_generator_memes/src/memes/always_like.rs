use skia_safe::{FontStyle, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    text_params,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn always_like(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let name = images[0].name.clone();
    let text = format!("我永远喜欢{name}");

    let frame = load_image("always_like/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();

    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(20, 470, frame.width() - 20, 570),
        text,
        30.0,
        70.0,
        text_params!(font_style = FontStyle::bold()),
    )?;
    let frame = surface.image_snapshot();

    let func = |images: &Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image = images[0].resize_fit((350, 400), Fit::Contain);
        canvas.draw_image(&image, (35, 55), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "always_like",
    always_like,
    min_images = 1,
    max_images = 1,
    keywords = &["我永远喜欢"],
    date_created = local_date(2022, 3, 14),
    date_modified = local_date(2024, 12, 17),
);
