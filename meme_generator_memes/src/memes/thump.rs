use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn thump(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let params = [
        (65, 128, 77, 72),
        (67, 128, 73, 72),
        (54, 139, 94, 61),
        (57, 135, 86, 65),
    ];
    let img = images[0].image.square();

    let mut encoder = GifEncoder::new();
    for i in 0..4 {
        let frame = load_image(format!("thump/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (x, y, w, h) = params[i];
        let img = img.resize_exact((w, h));
        canvas.draw_image(&img, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.04)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "thump",
    thump,
    min_images = 1,
    max_images = 1,
    keywords = &["捶"],
    date_created = local_date(2022, 3, 30),
    date_modified = local_date(2023, 2, 14),
);
