use skia_safe::{Color, FontStyle, IRect, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::{Fit, ImageExt},
    text_params,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn why_have_hands(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let name = &images[0].name;

    let mut text_surface = new_surface((600, 100));
    let canvas = text_surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(0, 0, 600, 100),
        &format!("摸摸{}!", name),
        30.0,
        70.0,
        text_params!(font_style = FontStyle::bold(), text_align = TextAlign::Left),
    )?;
    let text_image = text_surface.image_snapshot();

    let frame = load_image("why_have_hands/0.png")?;
    let mut surface = new_surface(frame.dimensions());
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    let img = &images[0].image;
    canvas.draw_image(
        &img.square().resize_exact((250, 250)).rotate_crop(-15.0),
        (1001, 668),
        None,
    );
    canvas.draw_image(img.resize_fit((250, 170), Fit::Cover), (275, 1100), None);
    canvas.draw_image(&frame, (0, 0), None);
    canvas.draw_image(&img.circle().resize_exact((250, 250)), (350, 670), None);
    canvas.draw_image(img.resize_fit((300, 400), Fit::Contain), (1100, 1060), None);

    canvas.draw_image(&text_image.rotate(15.0), (75, 825), None);
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(840, 960, 1440, 1060),
        &format!("托托{}!", name),
        30.0,
        70.0,
        text_params!(font_style = FontStyle::bold()),
    )?;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(50, 1325, 650, 1475),
        &format!("赞美{}!", name),
        30.0,
        90.0,
        text_params!(font_style = FontStyle::bold()),
    )?;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(700, 1340, 1075, 1490),
        &format!("为{}奉献所有财产!", name),
        30.0,
        70.0,
        text_params!(font_style = FontStyle::bold()),
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "why_have_hands",
    why_have_hands,
    min_images = 1,
    max_images = 1,
    keywords = &["为什么要有手"],
    date_created = local_date(2023, 5, 18),
    date_modified = local_date(2023, 5, 18),
);
