use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::encode_png,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn rip(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("rip/0.png")?;

    let img = if images.len() >= 2 {
        &images[1].image
    } else {
        &images[0].image
    };

    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    let img = img.square().resize_exact((385, 385));
    canvas.draw_image(&img.rotate(-24.0), (-5, 355), None);
    canvas.draw_image(&img.rotate(11.0), (649, 310), None);
    canvas.draw_image(&frame, (0, 0), None);

    if images.len() >= 2 {
        let img = images[0].image.circle().resize_exact((208, 208));
        canvas.draw_image(&img, (413, 422), None);
    }

    encode_png(surface.image_snapshot())
}

register_meme!(
    "rip",
    rip,
    min_images = 1,
    max_images = 2,
    keywords = &["撕"],
    date_created = local_date(2021, 5, 5),
    date_modified = local_date(2023, 2, 14),
);
