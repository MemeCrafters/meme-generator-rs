use rand::Rng;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::encode_png,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn throw(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(1..=360);
    let img = images[0]
        .image
        .circle()
        .rotate_crop(angle as f32)
        .resize_exact((143, 143));
    let frame = load_image("throw/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_image(&img, (15, 178), None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "throw",
    throw,
    min_images = 1,
    max_images = 1,
    keywords = &["丢", "扔"],
    tags = MemeTags::touhou(),
    date_created = local_date(2021, 5, 5),
    date_modified = local_date(2023, 3, 30),
);
