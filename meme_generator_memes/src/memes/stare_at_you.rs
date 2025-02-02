use skia_safe::{textlayout::TextAlign, Color, FontStyle, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    text_params,
    tools::{local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn stare_at_you(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let mut surface = new_surface((400, 400));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);

    canvas
        .draw_text_area(
            IRect::from_ltrb(0, 0, 150, 100),
            "我雇了一只",
            25.0,
            text_params!(
                text_align = TextAlign::Right,
                font_style = FontStyle::bold()
            ),
        )
        .unwrap();
    canvas
        .draw_text_area(
            IRect::from_ltrb(250, 0, 400, 100),
            "来盯着👁你",
            25.0,
            text_params!(text_align = TextAlign::Left, font_style = FontStyle::bold()),
        )
        .unwrap();
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let img = images[0].resize_fit((400, 300), Fit::Cover);
        let thumbnail = images[0].resize_fit((80, 60), Fit::Cover);
        canvas.draw_image(&img, (0, 100), None);
        canvas.draw_image(&thumbnail, (160, 20), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "stare_at_you",
    stare_at_you,
    min_images = 1,
    max_images = 1,
    keywords = &["盯着你"],
    date_created = local_date(2025, 1, 28),
    date_modified = local_date(2025, 2, 2),
);
