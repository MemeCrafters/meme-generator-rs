use skia_safe::IRect;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn you_should_call(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let name = &images[0].name;
    let text = format!("这种情况你应该致电{}", name);
    let frame = load_image("you_should_call/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(50, 20, 700, 130),
        &text,
        20.0,
        70.0,
        None,
    )?;

    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    let img = images[0].image.circle().resize_exact((300, 300));
    canvas.draw_image(&img, (400, 190), None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "you_should_call",
    you_should_call,
    min_images = 1,
    max_images = 1,
    keywords = &["致电", "你应该致电"],
    date_created = local_date(2024, 7, 26),
    date_modified = local_date(2024, 7, 26),
);
