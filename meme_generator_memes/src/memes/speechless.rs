use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    shortcut,
    text::Text2Image,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn speechless(images: Vec<NamedImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let mut text = "无语，和你说不下去".to_string();
    if !texts.is_empty() {
        text += &format!("\n{}", texts[0]);
    }
    let sweat = load_image("speechless/sweat.png")?.resize_exact((80, 80));

    let mut text_img = Text2Image::from_text(&text, 45.0, None);
    let text_w = 480;
    text_img.layout(text_w as f32);
    let text_h = text_img.height() as i32;

    let func = |images: Vec<Image>| {
        let frame_w = 500;
        let img = images[0].resize_width(frame_w);
        let frame_h = img.height() + text_h + 10;
        let mut surface = new_surface((frame_w, frame_h));
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        canvas.draw_image(&img, (0, 0), None);
        text_img.draw_on_canvas(&canvas, ((frame_w - text_w) / 2, img.height()));
        canvas.draw_image(&sweat, (300, 120), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "speechless",
    speechless,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["无语"],
    shortcuts = &[shortcut!(
        r"(?P<text>典型的\S+思维)",
        texts = &["${text}"],
        humanized = "典型的xx思维"
    ),],
    date_created = local_date(2024, 11, 12),
    date_modified = local_date(2024, 11, 12),
);
