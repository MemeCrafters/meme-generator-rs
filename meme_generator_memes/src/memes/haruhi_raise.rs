use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn haruhi_raise(images: Vec<DecodedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("haruhi_raise/0.png")?;

    let func = |images: &Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].resize_fit((250, 180), Fit::Cover);
        let image = image.perspective(&[(0, 51), (204, 0), (211, 147), (17, 217)]);
        canvas.draw_image(&image, (429, 79), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "haruhi_raise",
    haruhi_raise,
    min_images = 1,
    max_images = 1,
    keywords = &["凉宫春日举"],
    tags = MemeTags::haruhi(),
    date_created = local_date(2024, 11, 13),
    date_modified = local_date(2024, 11, 13),
);
