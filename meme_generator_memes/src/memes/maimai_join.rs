use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn maimai_join(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("maimai_join/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].square().resize_fit((400, 400), Fit::Cover);
        canvas.draw_image(&image, (50, 50), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "maimai_join",
    maimai_join,
    min_images = 1,
    max_images = 1,
    keywords = &["旅行伙伴加入"],
    tags = MemeTags::maimai(),
    date_created = local_date(2023, 7, 19),
    date_modified = local_date(2023, 7, 19),
);
