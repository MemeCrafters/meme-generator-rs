use skia_safe::IRect;

use crate::{
    error::Error,
    manager::register_meme,
    meme::DecodedImage,
    utils::{
        canvas::CanvasExt, encoder::encode_png, image::ImageExt, load_image, local_date,
        options::NoOptions,
    },
};

fn fanatic(
    _: &mut Vec<DecodedImage>,
    texts: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = &texts[0];

    let frame = load_image("fanatic/0.jpg")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(145, 40, 343, 160),
        text,
        30.0,
        70.0,
        None,
    )?;

    encode_png(&surface.image_snapshot())
}

register_meme!(
    "fanatic",
    fanatic,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["洛天依"],
    keywords = &["狂爱", "狂粉"],
    date_created = local_date(2021, 12, 15),
    date_modified = local_date(2023, 2, 14),
);
