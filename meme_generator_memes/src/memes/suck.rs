use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn suck(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (82, 100, 130, 119),
        (82, 94, 126, 125),
        (82, 120, 128, 99),
        (81, 164, 132, 55),
        (79, 163, 132, 55),
        (82, 140, 127, 79),
        (83, 152, 125, 67),
        (75, 157, 140, 62),
        (72, 165, 144, 54),
        (80, 132, 128, 87),
        (81, 127, 127, 92),
        (79, 111, 132, 108),
    ];
    let img = images[0].image.square();

    let mut encoder = GifEncoder::new();
    for i in 0..12 {
        let frame = load_image(format!("suck/{i:02}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (x, y, w, h) = locs[i];
        let img = img.resize_exact((w, h));
        canvas.draw_image(&img, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.08)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "suck",
    suck,
    min_images = 1,
    max_images = 1,
    keywords = &["吸", "嗦"],
    date_created = local_date(2022, 4, 20),
    date_modified = local_date(2023, 2, 14),
);
