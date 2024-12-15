use skia_safe::{Color, FontStyle, IRect, Image};

use crate::{
    error::Error,
    manager::register_meme,
    meme::DecodedImage,
    utils::{
        canvas::CanvasExt, encoder::make_png_or_gif, image::ImageExt, local_date, new_surface,
        options::Gender, text::TextParams,
    },
};

fn little_angel(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    options: &Gender,
) -> Result<Vec<u8>, Error> {
    let img_size = images[0].codec.dimensions();
    let img_w = 500;
    let img_h = img_size.height * img_w / img_size.width;
    let mut surface = new_surface((600, img_h + 230));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);

    let ta = match options.gender.as_str() {
        "male" => "他",
        _ => "她",
    };
    let mut name = images[0].name.as_str();
    if name.is_empty() {
        name = ta;
    }
    let text_params = TextParams {
        font_style: FontStyle::bold(),
        ..Default::default()
    };
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(20, 0, 580, 110),
        format!("请问你们看到{name}了吗?"),
        40.0,
        70.0,
        text_params.clone(),
    )?;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(10, img_h + 120, 590, img_h + 185),
        "非常可爱！简直就是小天使",
        40.0,
        48.0,
        text_params.clone(),
    )?;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(20, img_h + 180, 580, img_h + 215),
        format!("{ta}没失踪也没怎么样  我只是觉得你们都该看一下"),
        20.0,
        26.0,
        text_params.clone(),
    )?;
    let func = |images: &Vec<Image>| {
        let image = images[0].resize_width(img_w);
        let mut surface = surface.clone();
        let canvas = surface.canvas();
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
