use skia_safe::{Color, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

const DEFAULT_TEXT: &str = "平安名すみれ";

fn police(images: Vec<NamedImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = if texts.is_empty() {
        DEFAULT_TEXT
    } else {
        &texts[0]
    };
    let frame = load_image("police/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_xywh(220, 395, 250, 85),
        text,
        20.0,
        40.0,
        text_params!(font_families = &["Noto Serif SC"]),
    )?;
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let img = images[0].square().resize_exact((245, 245));
        canvas.draw_image(&img, (224, 46), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "police",
    police,
    min_images = 1,
    max_images = 1,
    max_texts = 1,
    min_texts = 0,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["出警"],
    date_created = local_date(2022, 2, 23),
    date_modified = local_date(2024, 9, 6),
);
