use skia_safe::{Color, Image};

use crate::{
    error::Error,
    manager::register_meme,
    meme::DecodedImage,
    utils::{
        encoder::make_png_or_gif,
        image::{Fit, ImageExt},
        load_image, local_date, new_surface,
        options::NoOptions,
    },
};

fn hold_tight(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("hold_tight/0.png")?;

    let func = |images: &Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].resize_fit((159, 171), Fit::Cover);
        canvas.draw_image(&image, (113, 205), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "hold_tight",
    hold_tight,
    min_images = 1,
    max_images = 1,
    keywords = &["抱紧"],
    date_created = local_date(2022, 10, 1),
    date_modified = local_date(2023, 2, 14),
);
