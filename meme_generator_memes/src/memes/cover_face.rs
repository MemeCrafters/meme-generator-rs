use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn cover_face(images: Vec<DecodedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |images: &Vec<Image>| {
        let frame = load_image("cover_face/0.png")?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].square().resize_exact((450, 450));
        let image = image.perspective(&[(15, 15), (448, 0), (445, 456), (0, 465)]);
        canvas.draw_image(&image, (120, 150), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "cover_face",
    cover_face,
    min_images = 1,
    max_images = 1,
    keywords = &["捂脸"],
    date_created = local_date(2022, 3, 30),
    date_modified = local_date(2023, 2, 14),
);
