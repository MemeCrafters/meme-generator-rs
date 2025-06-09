use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn washer(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("washer/0.png")?;
    let img = images[0].image.square();

    let mut encoder = GifEncoder::new();
    for i in 0..12 {
        let angle = i * 30;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let img = img.rotate_crop(angle as f32).resize_exact((74, 74));
        canvas.draw_image(&img, (63, 56), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.1)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "washer",
    washer,
    min_images = 1,
    max_images = 1,
    keywords = &["洗衣机"],
    date_created = local_date(2024, 1, 18),
    date_modified = local_date(2024, 1, 18),
);
