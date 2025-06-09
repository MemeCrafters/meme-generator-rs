use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};
fn garbage(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (39, 40),
        (39, 40),
        (39, 40),
        (39, 30),
        (39, 30),
        (39, 32),
        (39, 32),
        (39, 32),
        (39, 32),
        (39, 32),
        (39, 32),
        (39, 32),
        (39, 32),
        (39, 32),
        (39, 32),
        (39, 30),
        (39, 27),
        (39, 32),
        (37, 49),
        (37, 64),
        (37, 67),
        (37, 67),
        (39, 69),
        (37, 70),
        (37, 70),
    ];
    let image = images[0].image.square().resize_exact((79, 79));

    let mut encoder = GifEncoder::new();
    for i in 0..25 {
        let frame = load_image(format!("garbage/{i:02}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        canvas.draw_image(&image, locs[i], None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.04)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "garbage",
    garbage,
    min_images = 1,
    max_images = 1,
    keywords = &["垃圾", "垃圾桶"],
    date_created = local_date(2022, 4, 14),
    date_modified = local_date(2023, 2, 14),
);
