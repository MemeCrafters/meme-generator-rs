use rand::seq::SliceRandom;
use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::local_date,
};

use crate::{options::NoOptions, register_meme};

fn turn(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let direction = [-1, 1].choose(&mut rand::thread_rng()).unwrap();

    let func = |i: usize, images: Vec<Image>| {
        let angle = i as f32 * 10.0 * (*direction) as f32;
        let img = images[0]
            .circle()
            .rotate_crop(angle)
            .resize_exact((250, 250));
        Ok(img)
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 36,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "turn",
    turn,
    min_images = 1,
    max_images = 1,
    keywords = &["转"],
    date_created = local_date(2022, 1, 1),
    date_modified = local_date(2024, 9, 30),
);
