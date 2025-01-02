use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::{Fit, ImageExt},
    tools::{load_image, local_date},
};

use crate::{options::Circle, register_meme, tags::MemeTags};

fn kirby_hammer(
    images: Vec<NamedImage>,
    _: Vec<String>,
    options: Circle,
) -> Result<Vec<u8>, Error> {
    let positions = [
        (318, 163),
        (319, 173),
        (320, 183),
        (317, 193),
        (312, 199),
        (297, 212),
        (289, 218),
        (280, 224),
        (278, 223),
        (278, 220),
        (280, 215),
        (280, 213),
        (280, 210),
        (280, 206),
        (280, 201),
        (280, 192),
        (280, 188),
        (280, 184),
        (280, 179),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("kirby_hammer/{i:02}.png"))?;
        if i > 39 {
            return Ok(frame);
        }

        let mut image = images[0].clone();
        if options.circle.unwrap() {
            image = image.circle();
        }
        image = image.resize_height(80);
        if image.width() < 80 {
            image = image.resize_fit((80, 80), Fit::Cover);
        }
        let (x, y) = if i <= 18 { positions[i] } else { positions[18] };
        let pos = (x + 40 - image.width() / 2, y);
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        canvas.draw_image(&image, pos, None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 62,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "kirby_hammer",
    kirby_hammer,
    min_images = 1,
    max_images = 1,
    keywords = &["卡比锤", "卡比重锤"],
    tags = MemeTags::kirby(),
    date_created = local_date(2022, 11, 8),
    date_modified = local_date(2023, 2, 14),
);
