use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn knock(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (60, 308, 210, 195),
        (60, 308, 210, 198),
        (45, 330, 250, 172),
        (58, 320, 218, 180),
        (60, 310, 215, 193),
        (40, 320, 250, 285),
        (48, 308, 226, 192),
        (51, 301, 223, 200),
    ];
    let image = images[0].image.square();

    let mut encoder = GifEncoder::new();
    for i in 0..8 {
        let frame = load_image(format!("knock/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (x, y, w, h) = locs[i];
        let image = image.resize_exact((w, h));
        canvas.draw_image(&image, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.04)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "knock",
    knock,
    min_images = 1,
    max_images = 1,
    keywords = &["敲"],
    tags = MemeTags::gura(),
    date_created = local_date(2022, 4, 14),
    date_modified = local_date(2023, 2, 14),
);
