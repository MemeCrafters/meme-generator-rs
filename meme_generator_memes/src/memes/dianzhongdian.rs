use skia_safe::{Color, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    api::translate,
    builder::NamedImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    text_params,
    tools::{local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn dianzhongdian(
    images: Vec<NamedImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let (text, trans) = if texts.len() == 1 {
        let text = texts[0].clone();
        let trans = translate(&text, "jp");
        (text, trans)
    } else {
        (texts[0].clone(), Some(texts[1].clone()))
    };

    let img = &images[0].image;
    let img_w = 500;
    let img_h = img_w * img.height() / img.width();
    let frame_w = img_w;
    let frame_h = match trans {
        Some(_) => img_h + 100,
        None => img_h + 70,
    };

    let mut surface = new_surface((frame_w, frame_h));
    let canvas = surface.canvas();
    canvas.clear(Color::BLACK);
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(20, img_h, frame_w - 20, img_h + 60),
        &text,
        25.0,
        50.0,
        text_params!(paint = new_paint(Color::WHITE)),
    )?;
    if let Some(trans) = trans {
        canvas
            .draw_text_area_auto_font_size(
                IRect::from_ltrb(20, img_h + 60, frame_w - 20, img_h + 90),
                trans,
                10.0,
                25.0,
                text_params!(paint = new_paint(Color::WHITE)),
            )
            .map_err(|_| Error::TextOverLength(text.to_string()))?;
    };
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let img = images[0].resize_width(img_w).grayscale();
        canvas.draw_image(&img, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "dianzhongdian",
    dianzhongdian,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 2,
    default_texts = &["救命啊"],
    keywords = &["入典", "典中典", "黑白草图"],
    date_created = local_date(2022, 3, 12),
    date_modified = local_date(2023, 2, 14),
);
