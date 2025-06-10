use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn wallpaper(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let img = images[0].image.resize_fit((515, 383), Fit::Cover);

    let mut encoder = GifEncoder::new();
    for i in 0..20 {
        let frame = load_image(format!("wallpaper/{i:02}.png"))?;

        let frame = if i < 8 {
            frame
        } else {
            let mut surface = new_surface(frame.dimensions());
            let canvas = surface.canvas();
            canvas.clear(Color::WHITE);
            canvas.draw_image(&img, (176, -9), None);
            canvas.draw_image(&frame, (0, 0), None);
            surface.image_snapshot()
        };
        encoder.add_frame(frame, 0.07)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "wallpaper",
    wallpaper,
    min_images = 1,
    max_images = 1,
    keywords = &["墙纸"],
    tags = MemeTags::rick(),
    date_created = local_date(2022, 3, 9),
    date_modified = local_date(2023, 2, 14),
);
