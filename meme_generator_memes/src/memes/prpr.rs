use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn prpr(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("prpr/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let screen = images[0].resize_fit((330, 330), Fit::Cover);
        let screen = screen.perspective(&[(0, 19), (236, 0), (287, 264), (66, 351)]);
        canvas.draw_image(&screen, (56, 284), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "prpr",
    prpr,
    min_images = 1,
    max_images = 1,
    keywords = &["舔", "舔屏", "prpr"],
    date_created = local_date(2022, 3, 5),
    date_modified = local_date(2023, 2, 14),
);
