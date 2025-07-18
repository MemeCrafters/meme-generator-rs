use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn capoo_draw(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let params = [
        ([(27, 0), (207, 12), (179, 142), (0, 117)], (30, 16)),
        ([(28, 0), (207, 13), (180, 137), (0, 117)], (34, 17)),
    ];
    let mut raw_frames = vec![];
    for i in 0..6 {
        raw_frames.push(load_image(format!("capoo_draw/{i}.png"))?);
    }
    let indexes = [0, 1, 2, 1, 2, 3, 4, 5, 4, 5, 4, 5, 4, 5, 4, 5];
    let image = images[0].image.resize_fit((175, 120), Fit::Cover);

    let mut encoder = GifEncoder::new();
    for i in 0..16 {
        let index = indexes[i];
        let frame = &raw_frames[index];
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        if (4..6).contains(&index) {
            let (points, pos) = params[index - 4];
            let image = image.perspective(&points);
            canvas.draw_image(&image, pos, None);
        }
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.1)?;
    }
    Ok(encoder.finish()?)
}

register_meme! {
    "capoo_draw",
    capoo_draw,
    min_images = 1,
    max_images = 1,
    keywords = &["咖波画"],
    tags = MemeTags::capoo(),
    date_created = local_date(2023, 3, 31),
    date_modified = local_date(2023, 4, 28),
}
