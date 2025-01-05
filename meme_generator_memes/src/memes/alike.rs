use skia_safe::{textlayout::TextAlign, Color, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    text_params,
    tools::{local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn alike(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let mut surface = new_surface((470, 180));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);

    canvas
        .draw_text_area(
            IRect::from_ltrb(10, 30, 185, 160),
            "你怎么跟",
            40.0,
            text_params!(text_align = TextAlign::Right),
        )
        .unwrap();
    canvas
        .draw_text_area(
            IRect::from_ltrb(365, 30, 460, 160),
            "一样",
            40.0,
            text_params!(text_align = TextAlign::Left),
        )
        .unwrap();

    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image = images[0].resize_fit((150, 150), Fit::Cover);
        canvas.draw_image(&image, (200, 15), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "alike",
    alike,
    min_images = 1,
    max_images = 1,
    keywords = &["一样"],
    date_created = local_date(2022, 1, 2),
    date_modified = local_date(2023, 2, 22),
);
