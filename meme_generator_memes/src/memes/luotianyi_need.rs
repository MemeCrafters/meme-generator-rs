use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn luotianyi_need(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("luotianyi_need/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let img = images[0].resize_fit((195, 195), Fit::Contain);
        canvas.draw_image(&img, (43, 146), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "luotianyi_need",
    luotianyi_need,
    min_images = 1,
    max_images = 1,
    keywords = &["洛天依要", "天依要"],
    tags = MemeTags::luotianyi(),
    date_created = local_date(2025, 2, 11),
    date_modified = local_date(2025, 2, 11),
);
