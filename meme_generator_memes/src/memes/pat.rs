use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn pat(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [(11, 73, 106, 100), (8, 79, 112, 96)];
    let indexes = [
        0, 1, 2, 3, 1, 2, 3, 0, 1, 2, 3, 0, 0, 1, 2, 3, 0, 0, 0, 0, 4, 5, 5, 5, 6, 7, 8, 9,
    ];
    let img = images[0].image.square();

    let mut encoder = GifEncoder::new();
    for i in 0..28 {
        let index = indexes[i];
        let frame = load_image(format!("pat/{index}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (x, y, w, h) = if index == 2 { locs[1] } else { locs[0] };
        let img = img.resize_exact((w, h));
        canvas.draw_image(&img, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.085)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "pat",
    pat,
    min_images = 1,
    max_images = 1,
    keywords = &["拍"],
    date_created = local_date(2021, 12, 1),
    date_modified = local_date(2023, 2, 14),
);
