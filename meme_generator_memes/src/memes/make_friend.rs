use skia_safe::{Color, Image, Point};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    text::Text2Image,
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn make_friend(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let name = &images[0].name;
    let frame = load_image("make_friend/0.png")?;

    let angle = -9.0;
    let mut surface = new_surface(frame.dimensions());
    let canvas = surface.canvas();
    let img = images[0].image.square();
    canvas.draw_image(
        &img.resize_exact((250, 250)).rotate(angle),
        (743, frame.height() - 155),
        None,
    );
    canvas.draw_image(
        &img.resize_exact((55, 55)).rotate(angle),
        (836, frame.height() - 278),
        None,
    );
    let text2image =
        Text2Image::from_text(name, 20.0, text_params!(paint = new_paint(Color::WHITE)));
    if text2image.longest_line() > 230.0 {
        return Err(Error::TextOverLength(name.to_string()));
    }
    canvas.draw_image(&frame, (0, 0), None);
    canvas.rotate(angle, Some(Point::new(710.0, 710.0)));
    text2image.draw_on_canvas(&canvas, (710, 710 - text2image.height() as i32 / 2));
    canvas.reset_matrix();
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let img = images[0].resize_width(frame.width());
        canvas.draw_image(&img, (0, 0), None);
        canvas.draw_image(&frame, (0, img.height() - frame.height()), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "make_friend",
    make_friend,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["交个朋友"],
    date_created = local_date(2022, 3, 9),
    date_modified = local_date(2023, 2, 14),
);
