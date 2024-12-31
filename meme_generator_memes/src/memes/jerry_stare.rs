use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn jerry_stare(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("jerry_stare/0.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        let image = images[0].circle().resize_exact((150, 150));
        canvas.draw_image(&image, (184, 268), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "jerry_stare",
    jerry_stare,
    min_images = 1,
    max_images = 1,
    tags = MemeTags::jerry(),
    keywords = &["杰瑞盯"],
    date_created = local_date(2024, 8, 9),
    date_modified = local_date(2024, 8, 9),
);
