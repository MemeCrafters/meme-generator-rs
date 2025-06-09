use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn eat(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let image = images[0].image.square();
    let mut encoder = GifEncoder::new();
    for i in 0..3 {
        let frame = load_image(format!("eat/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = image.resize_exact((34, 34));
        canvas.draw_image(&image, (2, 38), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.05)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "eat",
    eat,
    min_images = 1,
    max_images = 1,
    keywords = &["吃"],
    date_created = local_date(2022, 2, 15),
    date_modified = local_date(2023, 2, 14),
);
