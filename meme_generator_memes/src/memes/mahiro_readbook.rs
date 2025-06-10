use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn mahiro_readbook(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let positions = [(0, 118), (0, 117), (0, 116), (0, 116), (-3, 116), (-7, 117)];
    let img = images[0]
        .image
        .resize_fit((70, 100), Fit::Cover)
        .perspective(&[(0, 6), (77, -5), (100, 100), (32, 100)]);

    let mut encoder = GifEncoder::new();
    for i in 0..48 {
        let frame = load_image(format!("mahiro_readbook/{i:02}.png"))?;
        let mut surface = new_surface((240, 240));
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let idx = (i - 16).max(0).min(5) as usize;
        canvas.draw_image(&img, positions[idx], None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.08)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "mahiro_readbook",
    mahiro_readbook,
    min_images = 1,
    max_images = 1,
    keywords = &["真寻看书"],
    tags = MemeTags::mahiro(),
    date_created = local_date(2024, 8, 18),
    date_modified = local_date(2024, 8, 18),
);
