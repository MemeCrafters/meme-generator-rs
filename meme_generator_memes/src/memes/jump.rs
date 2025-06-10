use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn jump(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (15, 50),
        (13, 43),
        (15, 23),
        (14, 4),
        (16, -3),
        (16, -4),
        (14, 4),
        (15, 31),
    ];
    let image = images[0].image.circle().resize_exact((40, 40));

    let mut encoder = GifEncoder::new();
    for i in 0..8 {
        let frame = load_image(format!("jump/{i}.png"))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        canvas.draw_image(&image, locs[i], None);
        encoder.add_frame(surface.image_snapshot(), 0.05)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "jump",
    jump,
    min_images = 1,
    max_images = 1,
    keywords = &["跳"],
    date_created = local_date(2024, 7, 14),
    date_modified = local_date(2024, 7, 14),
);
