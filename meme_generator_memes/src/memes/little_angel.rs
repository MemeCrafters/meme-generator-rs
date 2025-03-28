use skia_safe::{Color, FontStyle, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    text_params,
    tools::{local_date, new_surface},
};

use crate::{options::Gender, register_meme};

fn little_angel(
    images: Vec<InputImage>,
    _: Vec<String>,
    options: Gender,
) -> Result<Vec<u8>, Error> {
    let img = &images[0].image;
    let img_w = 500;
    let img_h = img.height() * img_w / img.width();
    let mut surface = new_surface((600, img_h + 230));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);

    let ta = match options.gender.as_deref().unwrap() {
        "male" => "他",
        _ => "她",
    };
    let mut name = images[0].name.as_str();
    if name.is_empty() {
        name = ta;
    }
    let text_params = text_params!(font_style = FontStyle::bold());
    canvas
        .draw_text_area_auto_font_size(
            IRect::from_ltrb(20, 20, 580, 110),
            format!("请问你们看到{name}了吗?"),
            40.0,
            70.0,
            text_params.clone(),
        )
        .map_err(|_| Error::TextOverLength(name.to_string()))?;
    canvas
        .draw_text_area(
            IRect::from_ltrb(10, img_h + 115, 590, img_h + 190),
            "非常可爱！简直就是小天使",
            48.0,
            text_params.clone(),
        )
        .unwrap();
    canvas
        .draw_text_area(
            IRect::from_ltrb(20, img_h + 175, 580, img_h + 220),
            format!("{ta}没失踪也没怎么样  我只是觉得你们都该看一下"),
            26.0,
            text_params.clone(),
        )
        .unwrap();
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image = images[0].resize_width(img_w);
        canvas.draw_image(&image, (300.0 - img_w as f32 / 2.0, 110.0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "little_angel",
    little_angel,
    min_images = 1,
    max_images = 1,
    keywords = &["小天使"],
    date_created = local_date(2022, 1, 1),
    date_modified = local_date(2023, 2, 14),
);
