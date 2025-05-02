use rand::Rng;
use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn throw(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |images: Vec<Image>| {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(1..=360);
        let img = images[0]
            .circle()
            .rotate_crop(angle as f32)
            .resize_exact((143, 143));
        let frame = load_image("throw/0.png")?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        canvas.draw_image(&img, (15, 178), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
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
