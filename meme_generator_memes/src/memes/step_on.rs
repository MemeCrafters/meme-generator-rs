use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn step_on(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let params = [
        (104, 72, 32, 185, 25),
        (104, 72, 32, 185, 25),
        (90, 73, 51, 207, 0),
        (88, 78, 51, 202, 0),
        (88, 86, 49, 197, 0),
    ];
    let img = images[0].image.square();

    let mut encoder = GifEncoder::new();
    for i in 0..5 {
        let frame = load_image(format!("step_on/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (w, h, x, y, angle) = params[i];
        let img = img.resize_exact((w, h)).rotate(angle as f32);
        canvas.draw_image(&img, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.07)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "step_on",
    step_on,
    min_images = 1,
    max_images = 1,
    keywords = &["踩"],
    date_created = local_date(2023, 3, 28),
    date_modified = local_date(2023, 3, 28),
);
