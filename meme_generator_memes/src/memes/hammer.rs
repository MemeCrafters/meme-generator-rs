use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn hammer(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (62, 143, 158, 113),
        (52, 177, 173, 105),
        (42, 192, 192, 92),
        (46, 182, 184, 100),
        (54, 169, 174, 110),
        (69, 128, 144, 135),
        (65, 130, 152, 124),
    ];
    let image = images[0].image.square();

    let mut encoder = GifEncoder::new();
    for i in 0..7 {
        let frame = load_image(format!("hammer/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        let (x, y, w, h) = locs[i];
        canvas.clear(Color::WHITE);
        let image = image.resize_exact((w, h));
        canvas.draw_image(&image, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.07)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "hammer",
    hammer,
    min_images = 1,
    max_images = 1,
    keywords = &["锤"],
    date_created = local_date(2022, 4, 20),
    date_modified = local_date(2023, 2, 14),
);
