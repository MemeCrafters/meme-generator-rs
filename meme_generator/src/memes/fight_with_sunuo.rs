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
        tags::MemeTags,
    },
};

fn fight_with_sunuo(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("fight_with_sunuo/0.png")?;

    let func = |images: &Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        let image = images[0].resize_fit((565, 1630), Fit::Cover).grayscale();
        canvas.draw_image(&image, (0, 245), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "fight_with_sunuo",
    fight_with_sunuo,
    min_images = 1,
    max_images = 1,
    keywords = &["我打宿傩", "我打宿傩吗"],
    tags = MemeTags::sukuna(),
    date_created = local_date(2024, 4, 3),
    date_modified = local_date(2024, 5, 25),
);
