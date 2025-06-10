use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn klee_eat(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (0, 174),
        (0, 174),
        (0, 174),
        (0, 174),
        (0, 174),
        (12, 160),
        (19, 152),
        (23, 148),
        (26, 145),
        (32, 140),
        (37, 136),
        (42, 131),
        (49, 127),
        (70, 126),
        (88, 128),
        (-30, 210),
        (-19, 207),
        (-14, 200),
        (-10, 188),
        (-7, 179),
        (-3, 170),
        (-3, 175),
        (-1, 174),
        (0, 174),
        (0, 174),
        (0, 174),
        (0, 174),
        (0, 174),
        (0, 174),
        (0, 174),
        (0, 174),
    ];
    let image = images[0].image.square().resize_exact((83, 83));

    let mut encoder = GifEncoder::new();
    for i in 0..31 {
        let frame = load_image(format!("klee_eat/{i:02}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        canvas.draw_image(&image, locs[i], None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.1)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "klee_eat",
    klee_eat,
    min_images = 1,
    max_images = 1,
    keywords = &["可莉吃"],
    tags = MemeTags::klee(),
    date_created = local_date(2022, 11, 29),
    date_modified = local_date(2023, 2, 14),
);
