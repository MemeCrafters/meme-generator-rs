use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn pepe_raise(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let left_locs = [
        (107, 30),
        (107, 30),
        (95, 45),
        (80, 160),
        (80, 160),
        (70, 98),
    ];
    let right_locs = [
        (320, 145),
        (320, 145),
        (330, 130),
        (300, 50),
        (300, 50),
        (323, 80),
    ];
    let left_img = images[0].image.circle().resize_exact((100, 100));
    let right_img = images[1].image.circle().resize_exact((100, 100));

    let mut encoder = GifEncoder::new();
    for i in 0..6 {
        let frame = load_image(format!("pepe_raise/{i}.png"))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        canvas.draw_image(&left_img, left_locs[i], None);
        canvas.draw_image(&right_img, right_locs[i], None);
        encoder.add_frame(surface.image_snapshot(), 0.06)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "pepe_raise",
    pepe_raise,
    min_images = 2,
    max_images = 2,
    keywords = &["佩佩举"],
    tags = MemeTags::arknights(),
    date_created = local_date(2024, 8, 18),
    date_modified = local_date(2024, 8, 18),
);
