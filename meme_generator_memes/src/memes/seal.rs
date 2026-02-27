use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn seal(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let width = images[0].image.square().width();
    let mask = load_image("seal/0.png")?;
    let mask = mask.resize_exact((width, width));

    let func = |images: Vec<Image>| {
        let img = &images[0].square();
        let mut surface = img.to_surface();
        let canvas = surface.canvas();
        canvas.draw_image(&mask, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "seal",
    seal,
    min_images = 1,
    max_images = 1,
    keywords = &["源石封印"],
    tags = MemeTags::arknights(),
    date_created = local_date(2025, 5, 25),
    date_modified = local_date(2025, 5, 25),
);
