use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn chino_throw(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let image = images[0].image.square();
    let locs = [
        (133, 299, 240, 186),
        (133, 299, 240, 186),
        (133, 299, 240, 186),
        (133, 299, 240, 186),
        (133, 299, 240, 186),
        (93, 260, 250, 220),
        (74, 226, 234, 228),
        (95, 176, 236, 226),
        (95, 176, 236, 226),
        (70, 45, 138, 123),
        (104, 51, 128, 112),
        (126, 57, 103, 95),
        (49, 23, 190, 146),
        (0, 0, 142, 163),
        (0, 0, 500, 500),
        (0, 0, 500, 500),
        (0, 0, 500, 500),
        (0, 0, 500, 500),
        (0, 0, 500, 500),
        (0, 0, 500, 500),
        (0, 0, 500, 500),
    ];

    let mut encoder = GifEncoder::new();
    for i in 0..21 {
        let frame = load_image(format!("chino_throw/{i:02}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (x, y, w, h) = locs[i];
        let image = image.resize_exact((w, h));
        canvas.draw_image(&image, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.07)?;
    }
    Ok(encoder.finish()?)
}

register_meme! {
    "chino_throw",
    chino_throw,
    min_images = 1,
    max_images = 1,
    keywords = &["智乃扔", "智乃抛"],
    tags = MemeTags::kafu_chino(),
    date_created = local_date(2025, 5, 23),
    date_modified = local_date(2025, 5, 23),
}
