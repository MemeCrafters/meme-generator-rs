use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn dog_of_vtb(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("dog_of_vtb/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].resize_fit((600, 450), Fit::Cover);
        let image = image.perspective(&[(0, 0), (579, 0), (584, 430), (5, 440)]);
        canvas.draw_image(&image, (97, 32), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme! {
    "dog_of_vtb",
    dog_of_vtb,
    min_images = 1,
    max_images = 1,
    keywords = &["管人痴"],
    date_created = local_date(2023, 4, 18),
    date_modified = local_date(2023, 4, 18),
}
