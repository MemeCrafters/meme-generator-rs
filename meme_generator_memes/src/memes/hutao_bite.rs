use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn hutao_bite(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [(98, 101, 108, 234), (96, 100, 108, 237)];
    let image = images[0].image.square();

    let mut encoder = GifEncoder::new();
    for i in 0..2 {
        let frame = load_image(format!("hutao_bite/{i}.png"))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (w, h, x, y) = locs[i];
        let image = image.resize_exact((w, h));
        canvas.draw_image(&image, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.1)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "hutao_bite",
    hutao_bite,
    min_images = 1,
    max_images = 1,
    keywords = &["胡桃啃"],
    tags = MemeTags::hutao(),
    date_created = local_date(2022, 11, 29),
    date_modified = local_date(2023, 2, 14),
);
