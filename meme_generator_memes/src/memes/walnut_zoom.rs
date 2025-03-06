use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn walnut_zoom(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let params = [
        (-222, 30, 695, 430),
        (-212, 30, 695, 430),
        (0, 30, 695, 430),
        (41, 26, 695, 430),
        (-100, -67, 922, 570),
        (-172, -113, 1059, 655),
        (-273, -192, 1217, 753),
    ];
    let seq = [
        0, 0, 0, 1, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 5, 6, 6, 6, 6,
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("walnut_zoom/{i:02}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (x, y, w, h) = params[seq[i]];
        let img = images[0].resize_fit((w, h), Fit::Cover).rotate(-4.2);
        canvas.draw_image(&img, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 24,
            duration: 0.2,
        },
        FrameAlign::ExtendLast,
    )
}

register_meme!(
    "walnut_zoom",
    walnut_zoom,
    min_images = 1,
    max_images = 1,
    tags = MemeTags::walnut(),
    keywords = &["胡桃放大"],
    date_created = local_date(2022, 10, 1),
    date_modified = local_date(2023, 2, 14),
);
