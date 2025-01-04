use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date},
};

use crate::{options::Gray, register_meme};

fn mourning(images: Vec<NamedImage>, _: Vec<String>, options: Gray) -> Result<Vec<u8>, Error> {
    let frame = load_image("mourning/0.png")?;
    let gray = options.gray.unwrap();

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let img = if gray {
            &images[0].grayscale()
        } else {
            &images[0]
        };
        let img = img.resize_fit((635, 725), Fit::Cover);
        canvas.draw_image(&img, (645, 145), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "mourning",
    mourning,
    min_images = 1,
    max_images = 1,
    keywords = &["上香"],
    date_created = local_date(2023, 7, 29),
    date_modified = local_date(2023, 7, 29),
);
