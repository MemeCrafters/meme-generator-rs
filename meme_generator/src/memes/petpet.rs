use serde::Deserialize;
use skia_safe::Image;

use crate::{
    encoder::encode_gif,
    error::Error,
    image::ImageExt,
    meme::{DecodedImage, MemeOptions},
    register_meme,
    utils::{load_image, local_date, new_surface},
};

#[derive(Debug, Clone, MemeOptions, Deserialize)]
#[serde(default)]
struct Options {
    /// 是否将图片变为圆形
    #[option(short, long, short_aliases = ['圆'])]
    circle: bool,
}

fn petpet(
    images: &Vec<DecodedImage>,
    _: &Vec<String>,
    options: &Options,
) -> Result<Vec<u8>, Error> {
    let image = &images[0].image;
    let mut image = image.square();
    if options.circle {
        image = image.circle();
    }
    let locs = [
        (14, 20, 98, 98),
        (12, 33, 101, 85),
        (8, 40, 110, 76),
        (10, 33, 102, 84),
        (12, 20, 98, 98),
    ];

    let mut frames: Vec<Image> = Vec::new();
    for i in 0..5 {
        let hand = load_image(format!("petpet/{i}.png").as_str())?;
        let mut surface = new_surface(hand.dimensions());
        let canvas = surface.canvas();
        let (x, y, w, h) = locs[i];
        let image = image.resize_exact((w, h));
        canvas.draw_image(&image, (x, y), None);
        canvas.draw_image(&hand, (0, 0), None);
        let frame = surface.image_snapshot();
        frames.push(frame);
    }
    encode_gif(&frames, 0.06)
}

register_meme! {
    "petpet",
    petpet,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 0,
    keywords = vec!["摸", "摸摸", "摸头", "rua"],
    date_created = local_date(2021, 8, 1),
    date_modified = local_date(2021, 8, 1),
}
