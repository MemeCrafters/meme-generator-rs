use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn dont_go_near(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("dont_go_near/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image = images[0].square().resize_exact((170, 170));
        canvas.draw_image(&image, (23, 231), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "dont_go_near",
    dont_go_near,
    min_images = 1,
    max_images = 1,
    keywords = &["不要靠近"],
    date_created = local_date(2022, 1, 2),
    date_modified = local_date(2023, 4, 20),
);
