use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn taunt(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("taunt/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let img = images[0].square().resize_exact((230, 230));
        canvas.draw_image(&img, (245, 245), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "taunt",
    taunt,
    min_images = 1,
    max_images = 1,
    keywords = &["嘲讽"],
    date_created = local_date(2023, 7, 19),
    date_modified = local_date(2023, 7, 19),
);
