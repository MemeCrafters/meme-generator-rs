use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn marriage(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let left = load_image("marriage/0.png")?;
    let right = load_image("marriage/1.png")?;

    let func = |images: Vec<Image>| {
        let img = images[0].resize_bound((1500, 1080), Fit::Contain);
        let mut surface = img.to_surface();
        let canvas = surface.canvas();
        canvas.draw_image(&left, (0, 0), None);
        canvas.draw_image(&right, (img.width() - right.width(), 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "marriage",
    marriage,
    min_images = 1,
    max_images = 1,
    keywords = &["结婚申请", "结婚登记"],
    date_created = local_date(2022, 5, 31),
    date_modified = local_date(2023, 2, 14),
);
