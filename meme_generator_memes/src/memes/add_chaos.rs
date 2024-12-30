use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn add_chaos(images: Vec<DecodedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let banner = load_image("add_chaos/0.png")?;

    let func = |images: &Vec<Image>| {
        let image = images[0].resize_width(240);
        let mut surface = image.to_surface();
        let canvas = surface.canvas();
        canvas.draw_image(&banner, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "add_chaos",
    add_chaos,
    min_images = 1,
    max_images = 1,
    keywords = &["添乱", "给社会添乱"],
    date_created = local_date(2023, 6, 21),
    date_modified = local_date(2023, 6, 21),
);
