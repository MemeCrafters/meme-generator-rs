use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn tightly(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let params = [
        (39, 169, 267, 141),
        (40, 167, 264, 143),
        (38, 174, 270, 135),
        (40, 167, 264, 143),
        (38, 174, 270, 135),
        (40, 167, 264, 143),
        (38, 174, 270, 135),
        (40, 167, 264, 143),
        (38, 174, 270, 135),
        (28, 176, 293, 134),
        (5, 215, 333, 96),
        (10, 210, 321, 102),
        (3, 210, 330, 104),
        (4, 210, 328, 102),
        (4, 212, 328, 100),
        (4, 212, 328, 100),
        (4, 212, 328, 100),
        (4, 212, 328, 100),
        (4, 212, 328, 100),
        (29, 195, 285, 120),
    ];
    let img = images[0].image.resize_fit((640, 400), Fit::Cover);

    let mut encoder = GifEncoder::new();
    for i in 0..20 {
        let frame = load_image(format!("tightly/{i:02}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (x, y, w, h) = params[i];
        let img = img.resize_exact((w, h));
        canvas.draw_image(&img, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.08)?;
    }
    Ok(encoder.finish()?)
}

register_meme! {
    "tightly",
    tightly,
    min_images = 1,
    max_images = 1,
    keywords = &["紧贴", "紧紧贴着"],
    date_created = local_date(2022, 4, 20),
    date_modified = local_date(2023, 2, 14),
}
