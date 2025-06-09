use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn trolley(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let img = images[0].image.circle();

    let mut encoder = GifEncoder::new();
    for i in 0..50 {
        let frame = load_image(format!("trolley/{i:02}.png"))?;
        let (w, h, x, y, angle) = if i < 25 {
            (65, 65, 21, 101, 0)
        } else if i < 28 {
            [
                (65, 65, 0, 101, 0),
                (65, 65, 0, 101, 0),
                (65, 65, -21, 101, 0),
            ][i - 25]
        } else if (31..=43).contains(&i) {
            [
                (18, 18, 237, 25, 0),
                (18, 18, 215, 25, 0),
                (18, 18, 191, 25, 0),
                (18, 18, 169, 25, 0),
                (18, 18, 150, 25, 0),
                (18, 18, 129, 19, -20),
                (18, 18, 114, 16, -30),
                (18, 18, 92, 13, -40),
                (18, 18, 72, 9, -40),
                (18, 18, 51, 6, -80),
                (18, 18, 27, 7, -90),
                (18, 18, 1, 8, -90),
                (18, 18, -15, 8, -90),
            ][i - 31]
        } else {
            encoder.add_frame(frame, 0.05)?;
            continue;
        };
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let img = img.resize_exact((w, h)).rotate_crop(angle as f32);
        canvas.draw_image(&img, (x, y), None);
        encoder.add_frame(surface.image_snapshot(), 0.05)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "trolley",
    trolley,
    min_images = 1,
    max_images = 1,
    keywords = &["推车"],
    date_created = local_date(2025, 4, 12),
    date_modified = local_date(2025, 4, 12),
);
