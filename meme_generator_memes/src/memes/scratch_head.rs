use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn scratch_head(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (53, 46, 4, 5),
        (50, 45, 7, 6),
        (50, 42, 6, 8),
        (50, 44, 7, 7),
        (53, 42, 4, 8),
        (52, 45, 7, 7),
    ];
    let img = images[0].image.square();

    let mut encoder = GifEncoder::new();
    for i in 0..6 {
        let frame = load_image(format!("scratch_head/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (w, h, x, y) = locs[i];
        let img = img.resize_exact((w, h));
        canvas.draw_image(&img, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.1)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "scratch_head",
    scratch_head,
    min_images = 1,
    max_images = 1,
    keywords = &["挠头"],
    date_created = local_date(2023, 1, 7),
    date_modified = local_date(2023, 2, 14),
);
