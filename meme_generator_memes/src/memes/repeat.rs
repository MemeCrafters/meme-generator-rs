use chrono::Local;
use skia_safe::{Color, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    text::Text2Image,
    text_params,
    tools::{color_from_hex_code, load_image, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn repeat(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let text_img = Text2Image::from_text(text, 50.0, text_params!(text_align = TextAlign::Left));
    if text_img.longest_line() > 900.0 {
        return Err(Error::TextOverLength(text.to_string()));
    }

    let time = Local::now().format("%H:%M").to_string();
    let time_img = Text2Image::from_text(
        time,
        40.0,
        text_params!(text_align = TextAlign::Left, paint = new_paint(Color::GRAY)),
    );

    let single_msg = |img: &Image, name: &str| {
        let user_img = img.circle().resize_exact((100, 100));
        let user_name_img =
            Text2Image::from_text(name, 40.0, text_params!(text_align = TextAlign::Left));

        let mut surface = new_surface((1079, 200));
        let canvas = surface.canvas();
        canvas.clear(color_from_hex_code("#f8f9fb"));
        canvas.draw_image(&user_img, (50, 50), None);
        user_name_img.draw_on_canvas(&canvas, (175, 45));
        time_img.draw_on_canvas(&canvas, (200 + user_name_img.longest_line() as i32, 50));
        text_img.draw_on_canvas(&canvas, (175, 100));
        surface.image_snapshot()
    };

    let mut surface = new_surface((1079, 1000));
    let canvas = surface.canvas();
    for i in 0..5 {
        let img = &images[i % images.len()];
        let msg_img = single_msg(&img.image, &img.name);
        canvas.draw_image(&msg_img, (0, 200 * i as i32), None);
    }
    let msg_img = surface.image_snapshot();

    let mut surface = new_surface((msg_img.width(), msg_img.height() * 2));
    let canvas = surface.canvas();
    canvas.draw_image(&msg_img, (0, 0), None);
    canvas.draw_image(&msg_img, (0, msg_img.height()), None);
    let msg_img_twice = surface.image_snapshot();

    let input_img = load_image("repeat/0.jpg")?;
    let self_img = images[0].image.circle().resize_exact((75, 75));
    let mut surface = input_img.to_surface();
    let canvas = surface.canvas();
    canvas.draw_image(&self_img, (15, 40), None);
    let input_img = surface.image_snapshot();

    let mut encoder = GifEncoder::new();
    for i in 0..50 {
        let mut surface = new_surface((1079, 1192));
        let canvas = surface.canvas();
        canvas.draw_image(&msg_img_twice, (0, -(20 * i)), None);
        canvas.draw_image(&input_img, (0, 1000), None);
        encoder.add_frame(surface.image_snapshot(), 0.08)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "repeat",
    repeat,
    min_images = 1,
    max_images = 5,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["救命啊"],
    keywords = &["复读"],
    date_created = local_date(2022, 6, 8),
    date_modified = local_date(2023, 2, 14),
);
