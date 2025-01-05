use skia_safe::{IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    shortcut,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn fill_head(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let name = if !images[0].name.is_empty() {
        &images[0].name
    } else {
        "它"
    };
    let text = format!("满脑子都是{name}");

    let frame = load_image("fill_head/0.jpg")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas
        .draw_text_area_auto_font_size(
            IRect::from_ltrb(20, 458, frame.width() - 20, 550),
            &text,
            30.0,
            65.0,
            None,
        )
        .map_err(|_| Error::TextOverLength(name.to_string()))?;
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image = images[0].resize_fit((210, 170), Fit::Cover);
        canvas.draw_image(&image, (150, 2), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "fill_head",
    fill_head,
    min_images = 1,
    max_images = 1,
    keywords = &["满脑子"],
    shortcuts = &[shortcut!(
        r"满脑子都是(?P<name>\S+)",
        names = &["${name}"],
        humanized = "满脑子都是xx"
    )],
    date_created = local_date(2023, 6, 3),
    date_modified = local_date(2023, 6, 3),
);
