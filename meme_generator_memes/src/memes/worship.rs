use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn worship(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let img = images[0].image.square().resize_exact((150, 150));

    let mut encoder = GifEncoder::new();
    for i in 0..10 {
        let frame = load_image(format!("worship/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let img = img.perspective(&[(0, -30), (135, 17), (135, 145), (0, 140)]);
        canvas.draw_image(&img, (0, 0), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.04)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "worship",
    worship,
    min_images = 1,
    max_images = 1,
    keywords = &["膜", "膜拜"],
    date_created = local_date(2022, 2, 10),
    date_modified = local_date(2023, 2, 14),
);
