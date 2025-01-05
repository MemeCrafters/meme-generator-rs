use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn out(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("out/0.png")?;

    let func = |images: Vec<Image>| {
        let image = &images[0];
        let frame = frame.resize_width((image.width() as f32 / 2.0) as i32);
        let mut surface = new_surface(image.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let pos_x = (image.width() - frame.width() - 10).min(image.width() * 4 / 10);
        let pos_y = (image.height() - frame.height() - 10).min(image.height() * 7 / 10);
        canvas.draw_image(&image, (0, 0), None);
        canvas.draw_image(&frame, (pos_x, pos_y), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "out",
    out,
    min_images = 1,
    max_images = 1,
    keywords = &["out"],
    date_created = local_date(2024, 4, 26),
    date_modified = local_date(2024, 4, 26),
);
