use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn applaud(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (109, 102, 27, 17),
        (107, 105, 28, 15),
        (110, 106, 27, 14),
        (109, 106, 27, 14),
        (107, 108, 29, 12),
    ];
    let image = images[0].image.square();

    let mut encoder = GifEncoder::new();
    for i in 0..5 {
        let frame = load_image(format!("applaud/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        let (w, h, x, y) = locs[i];
        let image = image.resize_exact((w, h));
        canvas.draw_image(&image, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.1)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "applaud",
    applaud,
    min_images = 1,
    max_images = 1,
    keywords = &["鼓掌"],
    date_created = local_date(2023, 1, 8),
    date_modified = local_date(2023, 2, 14),
);
