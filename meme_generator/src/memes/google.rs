use skia_safe::Color;

use crate::{
    error::Error,
    manager::register_meme,
    meme::DecodedImage,
    utils::{encoder::encode_png, local_date, new_surface, options::NoOptions, text::Text2Image},
};

fn google(_: &mut Vec<DecodedImage>, texts: &Vec<String>, _: &NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];

    let colors = [
        "#4285f4", "#db4437", "#f4b400", "#4285f4", "#0f9d58", "#db4437",
    ];
    let bbcode_text: String = text
        .chars()
        .enumerate()
        .map(|(i, char)| {
            if char.is_whitespace() {
                char.to_string()
            } else {
                format!("[color={}]{}[/color]", colors[i % colors.len()], char)
            }
        })
        .collect();
    let text2image = Text2Image::from_bbcode_text(&bbcode_text, 200.0, None);
    let mut surface = new_surface((
        text2image.longest_line() as i32 + 100,
        text2image.height() as i32 + 100,
    ));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    text2image.draw_on_canvas(canvas, (50, 50));

    encode_png(&surface.image_snapshot())
}

register_meme!(
    "google",
    google,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["Google"],
    keywords = &["google"],
    date_created = local_date(2022, 10, 30),
    date_modified = local_date(2023, 2, 14),
);
