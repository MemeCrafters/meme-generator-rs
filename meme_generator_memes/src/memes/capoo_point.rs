use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn capoo_point(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (165, 167, 57, 290),
        (165, 167, 53, 290),
        (160, 165, 57, 293),
        (165, 167, 56, 290),
    ];
    let image = images[0].image.square();

    let mut encoder = GifEncoder::new();
    for i in 0..4 {
        let frame = load_image(format!("capoo_point/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
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

register_meme! {
    "capoo_point",
    capoo_point,
    min_images = 1,
    max_images = 1,
    keywords = &["咖波指"],
    tags = MemeTags::capoo(),
    date_created = local_date(2024, 10, 24),
    date_modified = local_date(2024, 10, 24),
}
