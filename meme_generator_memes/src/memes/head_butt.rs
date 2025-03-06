use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn head_butt(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (180, 60, 100, 100),
        (184, 75, 100, 100),
        (183, 98, 100, 100),
        (179, 118, 110, 100),
        (156, 194, 150, 48),
        (178, 136, 122, 69),
        (175, 66, 122, 85),
        (170, 42, 130, 96),
        (175, 34, 118, 95),
        (179, 35, 110, 93),
        (180, 54, 102, 93),
        (183, 58, 97, 92),
        (174, 35, 120, 94),
        (179, 35, 109, 93),
        (181, 54, 101, 92),
        (182, 59, 98, 92),
        (183, 71, 90, 96),
        (180, 131, 92, 101),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let index = if (32..58).contains(&i) {
            i - 20
        } else if (24..32).contains(&i) {
            i - 24
        } else if (12..24).contains(&i) {
            i - 12
        } else {
            i
        };

        let frame = load_image(format!("head_butt/{index:02}.png"))?;
        if (0..18).contains(&index) {
            let mut surface = new_surface(frame.dimensions());
            let canvas = surface.canvas();
            canvas.clear(Color::WHITE);
            let (x, y, w, h) = locs[index];
            let img = images[0].square().resize_exact((w, h));
            canvas.draw_image(&img, (x, y), None);
            canvas.draw_image(&frame, (0, 0), None);
            Ok(surface.image_snapshot())
        } else {
            Ok(frame)
        }
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 58,
            duration: 0.06,
        },
        None,
    )
}

register_meme!(
    "head_butt",
    head_butt,
    min_images = 1,
    max_images = 1,
    keywords = &["顶"],
    tags = MemeTags::capoo(),
    date_created = local_date(2021, 10, 9),
    date_modified = local_date(2023, 2, 14),
);
