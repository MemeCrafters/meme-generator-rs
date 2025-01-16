use skia_safe::{FontStyle, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn together(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let name = &images[0].name;
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        &format!("一起玩{name}吧！")
    };

    let frame = load_image("together/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(10, 140, 190, 190),
        text,
        10.0,
        50.0,
        text_params!(font_style = FontStyle::bold()),
    )?;
    let frame = surface.image_snapshot();

    let func = |imgs: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let img = imgs[0].square().resize_exact((63, 63));
        canvas.draw_image(&img, (132, 36), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "together",
    together,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["一起"],
    date_created = local_date(2022, 10, 13),
    date_modified = local_date(2023, 3, 29),
);
