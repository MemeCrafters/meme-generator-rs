use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn twist(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let params = [
        (25, 66, 0),
        (25, 66, 60),
        (23, 68, 120),
        (20, 69, 180),
        (22, 68, 240),
        (25, 66, 300),
    ];
    let img = images[0].image.square();

    let mut encoder = GifEncoder::new();
    for i in 0..5 {
        let frame = load_image(format!("twist/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (x, y, a) = params[i];
        let img = img.rotate_crop(a as f32).resize_exact((78, 78));
        canvas.draw_image(&img, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.1)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "twist",
    twist,
    min_images = 1,
    max_images = 1,
    keywords = &["搓"],
    date_created = local_date(2022, 3, 9),
    date_modified = local_date(2023, 2, 14),
);
