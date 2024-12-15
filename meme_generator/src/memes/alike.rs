use skia_safe::{Color, IRect, Image};

use crate::{
    error::Error,
    manager::register_meme,
    meme::DecodedImage,
    utils::{
        canvas::CanvasExt,
        encoder::make_png_or_gif,
        image::{Fit, ImageExt},
        local_date, new_surface,
        options::NoOptions,
    },
};

fn alike(images: &mut Vec<DecodedImage>, _: &Vec<String>, _: &NoOptions) -> Result<Vec<u8>, Error> {
    let mut surface = new_surface((470, 180));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);

    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(10, 10, 185, 140),
        "你怎么跟",
        30.0,
        40.0,
        None,
    )?;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(365, 10, 460, 140),
        "一样",
        30.0,
        40.0,
        None,
    )?;

    let frame = surface.image_snapshot();

    let func = |images: &Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image = images[0].resize_fit((150, 150), Fit::Cover);
        canvas.draw_image(&image, (200.0, 15.0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "alike",
    alike,
    min_images = 1,
    max_images = 1,
    keywords = &["一样"],
    date_created = local_date(2022, 1, 2),
    date_modified = local_date(2023, 2, 22),
);
