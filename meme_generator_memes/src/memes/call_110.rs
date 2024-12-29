use skia_safe::{Color, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn call_110(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let mut surface = new_surface((900, 500));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(0, 0, 900, 200),
        "遇到困难请拨打",
        50.0,
        100.0,
        None,
    )?;
    let frame = surface.image_snapshot();

    let func = |images: &Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image1 = images[0].square().resize_exact((250, 250));
        let image0 = images[1].square().resize_exact((250, 250));
        canvas.draw_image(&image1, (50, 200), None);
        canvas.draw_image(&image1, (325, 200), None);
        canvas.draw_image(&image0, (600, 200), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme! {
    "call_110",
    call_110,
    min_images = 2,
    max_images = 2,
    keywords = &["遇到困难请拨打"],
    date_created = local_date(2022, 8, 26),
    date_modified = local_date(2023, 2, 14),
}
