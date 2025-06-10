use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn pound(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (135, 240, 138, 47),
        (135, 240, 138, 47),
        (150, 190, 105, 95),
        (150, 190, 105, 95),
        (148, 188, 106, 98),
        (146, 196, 110, 88),
        (145, 223, 112, 61),
        (145, 223, 112, 61),
    ];
    let img = images[0].image.square();

    let mut encoder = GifEncoder::new();
    for i in 0..8 {
        let frame = load_image(format!("pound/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (x, y, w, h) = locs[i];
        let img = img.resize_exact((w, h));
        canvas.draw_image(&img, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.05)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "pound",
    pound,
    min_images = 1,
    max_images = 1,
    keywords = &["捣"],
    date_created = local_date(2022, 3, 30),
    date_modified = local_date(2023, 2, 14),
);
