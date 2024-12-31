use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn konata_watch(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("konata_watch/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].resize_fit((270, 200), Fit::Cover);
        let image = image.perspective(&[(0, 1), (275, 0), (273, 202), (2, 216)]);
        canvas.draw_image(&image, (50, 188), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "konata_watch",
    konata_watch,
    min_images = 1,
    max_images = 1,
    keywords = &["泉此方看"],
    tags = MemeTags::konata(),
    date_created = local_date(2024, 8, 18),
    date_modified = local_date(2024, 8, 19),
);
