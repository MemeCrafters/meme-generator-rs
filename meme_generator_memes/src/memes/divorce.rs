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

fn divorce(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("divorce/0.png")?;

    let func = |images: &Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].resize_fit(frame.dimensions(), Fit::Cover);
        canvas.draw_image(&image, (0, 0), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "divorce",
    divorce,
    min_images = 1,
    max_images = 1,
    keywords = &["离婚协议", "离婚申请"],
    date_created = local_date(2023, 1, 7),
    date_modified = local_date(2023, 2, 14),
);
