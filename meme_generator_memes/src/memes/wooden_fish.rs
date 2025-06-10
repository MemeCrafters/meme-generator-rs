use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn wooden_fish(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let img = images[0].image.square().resize_exact((85, 85));

    let mut encoder = GifEncoder::new();
    for i in 0..66 {
        let frame = load_image(format!("wooden_fish/{i:02}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        canvas.draw_image(&img, (116, 153), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.1)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "wooden_fish",
    wooden_fish,
    min_images = 1,
    max_images = 1,
    keywords = &["木鱼"],
    date_created = local_date(2022, 11, 16),
    date_modified = local_date(2023, 2, 14),
);
