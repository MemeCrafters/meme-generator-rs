use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn forbid(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("forbid/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        let img = images[0].resize_fit(frame.dimensions(), Fit::Cover);
        canvas.draw_image(&img, (0, 0), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "forbid",
    forbid,
    min_images = 1,
    max_images = 1,
    keywords = &["禁止", "禁"],
    date_created = local_date(2023, 3, 12),
    date_modified = local_date(2023, 3, 12),
);
