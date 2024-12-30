use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn dinosaur(images: Vec<DecodedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("dinosaur/0.png")?;

    let func = |images: &Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        let image = images[0].resize_fit((680, 578), Fit::Cover);
        canvas.draw_image(&image, (294, 369), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "dinosaur",
    dinosaur,
    min_images = 1,
    max_images = 1,
    keywords = &["恐龙", "小恐龙"],
    date_created = local_date(2023, 1, 6),
    date_modified = local_date(2023, 2, 14),
);
