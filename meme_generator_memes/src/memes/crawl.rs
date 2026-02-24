use rand::RngExt;
use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::number_option, register_meme};

number_option!(Number, 1, 92);

fn crawl(images: Vec<InputImage>, _: Vec<String>, options: Number) -> Result<Vec<u8>, Error> {
    let num = options.number.unwrap_or({
        let mut rng = rand::rng();
        rng.random_range(1..=92)
    });

    let func = |images: Vec<Image>| {
        let frame = load_image(format!("crawl/{:02}.jpg", num))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image = images[0].circle().resize_exact((100, 100));
        canvas.draw_image(&image, (0, 400), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "crawl",
    crawl,
    min_images = 1,
    max_images = 1,
    keywords = &["爬"],
    date_created = local_date(2021, 5, 5),
    date_modified = local_date(2023, 2, 14),
);
