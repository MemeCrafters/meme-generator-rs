use skia_safe::Image;

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

fn anti_kidnap(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("anti_kidnap/0.png")?;

    let func = |images: &Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        let img = images[0].resize_fit((450, 450), Fit::Cover);
        canvas.draw_image(&img, (30, 78), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "anti_kidnap",
    anti_kidnap,
    min_images = 1,
    max_images = 1,
    keywords = &["防诱拐"],
    date_created = local_date(2022, 7, 9),
    date_modified = local_date(2023, 2, 14),
);
