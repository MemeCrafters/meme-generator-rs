use skia_safe::IRect;

use crate::{
    error::Error,
    manager::register_meme,
    meme::DecodedImage,
    utils::{
        canvas::CanvasExt, encoder::encode_png, image::ImageExt, load_image, options::NoOptions,
    },
};

fn dont_press(
    _: &mut Vec<DecodedImage>,
    texts: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = &texts[0];

    let frame = load_image("dont_press/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(60, 170, 200, 225),
        text,
        20.0,
        50.0,
        None,
    )?;

    encode_png(&surface.image_snapshot())
}

register_meme!(
    "dont_press",
    dont_press,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["世界毁灭"],
    keywords = &["不要按"],
);
