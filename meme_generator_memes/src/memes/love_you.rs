use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn love_you(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [(68, 65, 70, 70), (63, 59, 80, 80)];
    let image = images[0].image.square();

    let mut encoder = GifEncoder::new();
    for i in 0..2 {
        let frame = load_image(format!("love_you/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (x, y, w, h) = locs[i];
        let image = image.resize_exact((w, h));
        canvas.draw_image(&image, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.2)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "love_you",
    love_you,
    min_images = 1,
    max_images = 1,
    keywords = &["永远爱你"],
    date_created = local_date(2022, 3, 13),
    date_modified = local_date(2023, 2, 14),
);
