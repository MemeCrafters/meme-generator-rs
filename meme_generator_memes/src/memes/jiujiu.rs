use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn jiujiu(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let image = images[0].image.resize_fit((75, 51), Fit::Cover);
    let mut encoder = GifEncoder::new();
    for i in 0..8 {
        let frame = load_image(format!("jiujiu/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        canvas.draw_image(&image, (0, 0), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.06)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "jiujiu",
    jiujiu,
    min_images = 1,
    max_images = 1,
    keywords = &["啾啾"],
    date_created = local_date(2022, 4, 20),
    date_modified = local_date(2023, 2, 14),
);
