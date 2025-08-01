use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::encode_png,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn tomb_yeah(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("tomb_yeah/0.jpg")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    let img = images[0].image.circle().resize_exact((145, 145));
    canvas.draw_image(&img, (138, 265), None);
    if images.len() > 1 {
        let img = images[1]
            .image
            .circle()
            .rotate_crop(-30.0)
            .resize_exact((145, 145));
        canvas.draw_image(&img, (371, 312), None);
    }
    encode_png(surface.image_snapshot())
}

register_meme!(
    "tomb_yeah",
    tomb_yeah,
    min_images = 1,
    max_images = 2,
    keywords = &["上坟", "坟前比耶"],
    date_created = local_date(2023, 11, 12),
    date_modified = local_date(2023, 11, 12),
);
