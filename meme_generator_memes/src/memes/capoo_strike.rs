use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn capoo_strike(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let params = [
        ([(0, 4), (153, 0), (138, 105), (0, 157)], (28, 47)),
        ([(1, 13), (151, 0), (130, 104), (0, 156)], (28, 48)),
        ([(9, 10), (156, 0), (152, 108), (0, 155)], (18, 51)),
        ([(0, 21), (150, 0), (146, 115), (7, 145)], (17, 53)),
        ([(0, 19), (156, 0), (199, 109), (31, 145)], (2, 62)),
        ([(0, 28), (156, 0), (171, 115), (12, 154)], (16, 58)),
        ([(0, 25), (157, 0), (169, 113), (13, 147)], (18, 63)),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("capoo_strike/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].resize_fit((200, 160), Fit::Cover);
        let (points, pos) = params[i];
        let image = image.perspective(&points);
        canvas.draw_image(&image, pos, None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 7,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "capoo_strike",
    capoo_strike,
    min_images = 1,
    max_images = 1,
    tags = MemeTags::capoo(),
    keywords = &["咖波撞", "咖波头槌"],
    date_created = local_date(2023, 3, 28),
    date_modified = local_date(2023, 3, 28),
}
