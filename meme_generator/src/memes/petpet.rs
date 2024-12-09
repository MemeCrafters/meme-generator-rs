use serde::Deserialize;
use skia_safe::Image;

use crate::{
    encoder::{make_gif_or_combined_gif, GifInfo},
    error::Error,
    image::ImageExt,
    meme::{DecodedImage, MemeOptions},
    register_meme,
    utils::{load_image, local_date, new_surface},
};

#[derive(MemeOptions, Deserialize)]
#[serde(default)]
struct Options {
    /// 是否将图片变为圆形
    #[option(short, long, short_aliases = ['圆'])]
    circle: bool,
}

fn petpet(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    options: &Options,
) -> Result<Vec<u8>, Error> {
    let locs = [
        (14, 20, 98, 98),
        (12, 33, 101, 85),
        (8, 40, 110, 76),
        (10, 33, 102, 84),
        (12, 20, 98, 98),
    ];

    let func = |i: u32, images: &Vec<Image>| {
        let mut image = images[0].square();
        if options.circle {
            image = image.circle();
        }

        let hand = load_image(format!("petpet/{i}.png"))?;
        let mut surface = new_surface(hand.dimensions());
        let canvas = surface.canvas();
        let (x, y, w, h) = locs[i as usize];
        let image = image.resize_exact((w, h));
        canvas.draw_image(&image, (x, y), None);
        canvas.draw_image(&hand, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    let images = vec![&mut images[0].codec];
    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 5,
            duration: 0.06,
        },
        None,
    )
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
