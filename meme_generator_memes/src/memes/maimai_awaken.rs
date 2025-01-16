use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn maimai_awaken(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("maimai_awaken/0.png")?;

    let func = |images: Vec<Image>| {
        let img = images[0].square().resize_exact((250, 250)).rotate(25.0);
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        canvas.draw_image(&img, (134, 134), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "maimai_awaken",
    maimai_awaken,
    min_images = 1,
    max_images = 1,
    keywords = &["旅行伙伴觉醒"],
    tags = MemeTags::maimai(),
    date_created = local_date(2023, 7, 19),
    date_modified = local_date(2023, 7, 19),
);
