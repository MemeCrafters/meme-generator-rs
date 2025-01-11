use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn erised_mirror(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("erised_mirror/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let img = images[0]
            .resize_fit((360, 207), Fit::Cover)
            .colorize(Color::from_rgb(57, 78, 125))
            .brightness(0.7)
            .perspective(&[(0, 0), (360, 0), (367, 207), (7, 207)]);
        canvas.draw_image(&img, (55, 578), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "erised_mirror",
    erised_mirror,
    min_images = 1,
    max_images = 1,
    keywords = &["意若思镜"],
    tags = MemeTags::harry_potter(),
    date_created = local_date(2024, 8, 31),
    date_modified = local_date(2024, 8, 31),
);
