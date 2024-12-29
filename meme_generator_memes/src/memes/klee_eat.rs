use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn klee_eat(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let locs = [
        (0, 174),
        (0, 174),
        (0, 174),
        (0, 174),
        (0, 174),
        (12, 160),
        (19, 152),
        (23, 148),
        (26, 145),
        (32, 140),
        (37, 136),
        (42, 131),
        (49, 127),
        (70, 126),
        (88, 128),
        (-30, 210),
        (-19, 207),
        (-14, 200),
        (-10, 188),
        (-7, 179),
        (-3, 170),
        (-3, 175),
        (-1, 174),
        (0, 174),
        (0, 174),
        (0, 174),
        (0, 174),
        (0, 174),
        (0, 174),
        (0, 174),
        (0, 174),
    ];

    let func = |i: usize, images: &Vec<Image>| {
        let frame = load_image(format!("klee_eat/{i:02}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].square().resize_exact((83, 83));
        canvas.draw_image(&image, locs[i], None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 31,
            duration: 0.1,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "klee_eat",
    klee_eat,
    min_images = 1,
    max_images = 1,
    keywords = &["可莉吃"],
    tags = MemeTags::klee(),
    date_created = local_date(2022, 11, 29),
    date_modified = local_date(2023, 2, 14),
);
