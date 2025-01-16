use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn luoyonghao_say(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let frame = load_image("luoyonghao_say/0.jpg")?;

    let mut surface = new_surface((365, 120));
    let canvas = surface.canvas();
    let padding = 40;
    canvas.draw_text_area_auto_font_size(
        skia_safe::IRect::from_ltrb(padding, 10, 325, 110),
        text,
        10.0,
        50.0,
        None,
    )?;
    let text_image = surface.image_snapshot();
    let text_image = text_image
        .perspective(&[(52, 10), (391, 0), (364, 110), (0, 120)])
        .gaussian_blur(0.8);

    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_image(&text_image, (48, 246), None);

    encode_png(surface.image_snapshot())
}

register_meme!(
    "luoyonghao_say",
    luoyonghao_say,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["又不是不能用"],
    keywords = &["罗永浩说"],
    date_created = local_date(2023, 3, 28),
    date_modified = local_date(2023, 3, 28),
);
