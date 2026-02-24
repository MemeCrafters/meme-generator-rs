use rand::RngExt;
use skia_safe::{Color, FontStyle, IRect};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint, new_stroke_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn operator_generator(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let img = &images[0].image.circle().resize_exact((80, 80));
    let name = &images[0].name;

    let mut surface = new_surface((640, 640));
    let canvas = surface.canvas();
    canvas.clear(Color::from_rgb(160, 160, 160));
    canvas.draw_image(&img, (20, 10), None);
    canvas
        .draw_text_area_auto_font_size(
            IRect::from_ltrb(120, 0, 620, 100),
            format!("{name}，你的干员信息如下："),
            30.0,
            80.0,
            text_params!(
                paint = new_paint(Color::WHITE),
                stroke_paint = new_stroke_paint(Color::BLACK, 15.0),
                font_style = FontStyle::bold(),
            ),
        )
        .map_err(|_| Error::TextOverLength(name.clone()))?;

    let mut rng = rand::rng();

    let range = load_image(&format!(
        "operator_generator/range/{:02}.jpg",
        rng.random_range(0..=24)
    ))?
    .resize_width(320);
    canvas.draw_image(&range, (0, 100), None);

    let characteristic = load_image(&format!(
        "operator_generator/characteristic/{:02}.jpg",
        rng.random_range(0..=24)
    ))?
    .resize_width(320);
    canvas.draw_image(&characteristic, (320, 100), None);

    let value = load_image(&format!(
        "operator_generator/value/{:02}.jpg",
        rng.random_range(0..=24)
    ))?
    .resize_width(320);
    canvas.draw_image(&value, (0, 280), None);

    let talent = load_image(&format!(
        "operator_generator/talent/{:02}.jpg",
        rng.random_range(0..=24)
    ))?
    .resize_width(320);
    canvas.draw_image(&talent, (320, 280), None);

    let skill = load_image(&format!(
        "operator_generator/skill/{:02}.jpg",
        rng.random_range(0..=24)
    ))?
    .resize_width(320);
    canvas.draw_image(&skill, (0, 460), None);

    let special = load_image(&format!(
        "operator_generator/special/{:02}.jpg",
        rng.random_range(0..=24)
    ))?
    .resize_width(320);
    canvas.draw_image(&special, (320, 460), None);

    encode_png(surface.image_snapshot())
}

register_meme!(
    "operator_generator",
    operator_generator,
    min_images = 1,
    max_images = 1,
    keywords = &["合成大干员"],
    date_created = local_date(2023, 3, 28),
    date_modified = local_date(2023, 3, 28),
);
