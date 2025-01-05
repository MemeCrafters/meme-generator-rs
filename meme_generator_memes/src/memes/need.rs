use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn need(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("need/0.png")?;

    let func = |images: Vec<Image>| {
        let img = images[0].square().resize_exact((115, 115));
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        canvas.draw_image(&img, (327, 232), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "need",
    need,
    min_images = 1,
    max_images = 1,
    keywords = &["需要", "你可能需要"],
    date_created = local_date(2022, 3, 30),
    date_modified = local_date(2023, 2, 14),
);
