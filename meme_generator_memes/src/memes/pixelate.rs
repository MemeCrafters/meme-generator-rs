use skia_safe::{Image, SamplingOptions};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{MemeOptions, NamedImage},
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::local_date,
};

use crate::register_meme;

#[derive(MemeOptions)]
struct Number {
    /// 像素化大小
    #[option(short, long, minimum = 2, maximum = 50, default = 10)]
    number: Option<i32>,
}

fn pixelate(images: Vec<NamedImage>, _: Vec<String>, options: Number) -> Result<Vec<u8>, Error> {
    let num = options.number.unwrap();

    let func = |images: Vec<Image>| {
        let image = &images[0];
        let img = image
            .resize_exact_with_sampling_options(
                ((image.width() / num).max(1), (image.height() / num).max(1)),
                SamplingOptions::default(),
            )
            .resize_exact_with_sampling_options(image.dimensions(), SamplingOptions::default());
        Ok(img)
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "pixelate",
    pixelate,
    min_images = 1,
    max_images = 1,
    keywords = &["像素化"],
    date_created = local_date(2024, 8, 12),
    date_modified = local_date(2024, 8, 12),
);
