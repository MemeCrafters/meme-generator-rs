use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn daynight(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("daynight/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image1 = images[0].resize_fit((333, 360), Fit::Cover);
        let image2 = images[1].resize_fit((333, 360), Fit::Cover);
        canvas.draw_image(&image1, (349, 0), None);
        canvas.draw_image(&image2, (349, 361), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "daynight",
    daynight,
    min_images = 2,
    max_images = 2,
    keywords = &["白天黑夜", "白天晚上"],
    date_created = local_date(2023, 10, 3),
    date_modified = local_date(2023, 10, 3),
);
