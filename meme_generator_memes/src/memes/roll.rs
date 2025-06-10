use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn roll(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (87, 77, 0),
        (96, 85, 45),
        (92, 79, 90),
        (92, 78, 135),
        (92, 75, 180),
        (92, 75, 225),
        (93, 76, 270),
        (90, 80, 315),
    ];
    let img = images[0].image.square().resize_exact((210, 210));

    let mut encoder = GifEncoder::new();
    for i in 0..8 {
        let frame = load_image(format!("roll/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (x, y, a) = locs[i];
        let img = img.rotate_crop(a as f32);
        canvas.draw_image(&img, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.1)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "roll",
    roll,
    min_images = 1,
    max_images = 1,
    keywords = &["滚"],
    date_created = local_date(2022, 1, 4),
    date_modified = local_date(2023, 2, 14),
);
