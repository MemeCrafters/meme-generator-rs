use rand::Rng;
use skia_safe::Image;

use crate::{
    error::Error,
    manager::register_meme,
    meme::{DecodedImage, MemeOptions},
    utils::{encoder::make_png_or_gif, image::ImageExt, load_image, local_date},
};

#[derive(MemeOptions)]
struct Number {
    /// 图片编号
    #[option(short, long, minimum = 0, maximum = 92)]
    number: i32,
}

fn crawl(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    options: &Number,
) -> Result<Vec<u8>, Error> {
    let mut num = options.number;
    if num == 0 {
        let mut rng = rand::thread_rng();
        num = rng.gen_range(1..=92);
    }

    let func = |images: &Vec<Image>| {
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
