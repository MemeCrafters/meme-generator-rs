use skia_safe::{Color, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    text_params,
    tools::{load_image, local_date, new_paint, new_stroke_paint, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

const DEFAULT_TEXT: &str = "我老婆";

fn teach(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };

    let frame = load_image("teach/0.png")?.resize_width(960);
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(
            10,
            frame.height() - 80,
            frame.width() - 10,
            frame.height() - 5,
        ),
        text,
        20.0,
        50.0,
        text_params!(
            paint = new_paint(Color::WHITE),
            stroke_paint = new_stroke_paint(Color::BLACK, 6.0)
        ),
    )?;
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let img = images[0].resize_fit((550, 395), Fit::Cover);
        canvas.draw_image(&img, (313, 60), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "teach",
    teach,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["讲课", "敲黑板"],
    tags = MemeTags::takina(),
    date_created = local_date(2022, 8, 16),
    date_modified = local_date(2023, 2, 14),
);
