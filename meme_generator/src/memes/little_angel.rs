use skia_safe::{textlayout::TextAlign, Color, FontStyle, IRect, Image};

use crate::{
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    error::Error,
    image::ImageExt,
    meme::DecodedImage,
    options::common_used_options::Gender,
    register_meme,
    text::TextParams,
    utils::{local_date, new_surface},
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
        text_align: TextAlign::Center,
        ..Default::default()
    };
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(20, 0, 580, 110),
        format!("请问你们看到{name}了吗?"),
        70.0,
        25.0,
        &text_params,
    )?;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(10, img_h + 120, 590, img_h + 185),
        "非常可爱！简直就是小天使",
        48.0,
        40.0,
        &text_params,
    )?;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(20, img_h + 180, 580, img_h + 215),
        format!("{ta}没失踪也没怎么样  我只是觉得你们都该看一下"),
        26.0,
        20.0,
        &text_params,
    )?;
    let func = |images: &Vec<Image>| {
        let image = images[0].resize_width(img_w);
        let mut surface = surface.clone();
        let canvas = surface.canvas();
        canvas.draw_image(&image, (300.0 - img_w as f32 / 2.0, 110.0), None);
        Ok(surface.image_snapshot())
    };

    let images = vec![&mut images[0].codec];
    make_png_or_gif(images, func)
}

register_meme! {
    "little_angel",
    little_angel,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 0,
    keywords = vec!["小天使"],
    date_created = local_date(2022, 1, 1),
    date_modified = local_date(2023, 2, 14),
}
