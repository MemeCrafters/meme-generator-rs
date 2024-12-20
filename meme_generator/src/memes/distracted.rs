use skia_safe::Image;

use crate::{
    error::Error,
    manager::register_meme,
    meme::DecodedImage,
    tags::MemeTags,
    utils::{
        encoder::make_png_or_gif, image::ImageExt, load_image, local_date, options::NoOptions,
    },
};

fn distracted(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let frame = load_image("distracted/1.png")?;
    let label = load_image("distracted/0.png")?;

    let func = |images: &Vec<Image>| {
        let image = images[0].square().resize_exact((500, 500));
        let mut surface = image.to_surface();
        let canvas = surface.canvas();
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_image(&label, (140, 320), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "distracted",
    distracted,
    min_images = 1,
    max_images = 1,
    keywords = &["注意力涣散"],
    tags = MemeTags::arknights(),
    date_created = local_date(2022, 4, 20),
    date_modified = local_date(2023, 2, 14),
);
