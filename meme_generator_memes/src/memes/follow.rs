use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    text::Text2Image,
    text_params,
    tools::{local_date, new_paint, new_surface},
};

use crate::{options::Gender, register_meme};

fn follow(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    options: &Gender,
) -> Result<Vec<u8>, Error> {
    let name = if !images[0].name.is_empty() {
        &images[0].name
    } else {
        match options.gender.as_str() {
            "female" => "女同",
            _ => "男同",
        }
    };

    let name_image = Text2Image::from_text(name, 60.0, None);
    let follow_image = Text2Image::from_text(
        "关注了你",
        60.0,
        text_params!(paint = new_paint(Color::GRAY)),
    );
    let text_width = name_image.longest_line().max(follow_image.longest_line());
    if text_width >= 1000.0 {
        return Err(Error::TextOverLength(name.to_string()));
    }

    let frame_w = 300 + text_width as i32 + 50;
    let frame_h = 300;
    let mut surface = new_surface((frame_w, frame_h));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    name_image.draw_on_canvas(canvas, (300.0, 135.0 - name_image.height() as f32));
    follow_image.draw_on_canvas(canvas, (300.0, 145.0));
    let frame = surface.image_snapshot();

    let func = |images: &Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image = images[0].circle().resize_exact((200, 200));
        canvas.draw_image(&image, (50, 50), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "follow",
    follow,
    min_images = 1,
    max_images = 1,
    keywords = &["关注"],
    date_created = local_date(2022, 3, 10),
    date_modified = local_date(2023, 2, 14),
);
