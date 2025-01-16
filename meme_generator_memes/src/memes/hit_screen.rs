use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn hit_screen(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let params = [
        ([(1, 10), (138, 1), (140, 119), (7, 154)], (32, 37)),
        ([(1, 10), (138, 1), (140, 121), (7, 154)], (32, 37)),
        ([(1, 10), (138, 1), (139, 125), (10, 159)], (32, 37)),
        ([(1, 12), (136, 1), (137, 125), (8, 159)], (34, 37)),
        ([(1, 9), (137, 1), (139, 122), (9, 154)], (35, 41)),
        ([(1, 8), (144, 1), (144, 123), (12, 155)], (30, 45)),
        ([(1, 8), (140, 1), (141, 121), (10, 155)], (29, 49)),
        ([(1, 9), (140, 1), (139, 118), (10, 153)], (27, 53)),
        ([(1, 7), (144, 1), (145, 117), (13, 153)], (19, 57)),
        ([(1, 7), (144, 1), (143, 116), (13, 153)], (19, 57)),
        ([(1, 8), (139, 1), (141, 119), (12, 154)], (19, 55)),
        ([(1, 13), (140, 1), (143, 117), (12, 156)], (16, 57)),
        ([(1, 10), (138, 1), (142, 117), (11, 149)], (14, 61)),
        ([(1, 10), (141, 1), (148, 125), (13, 153)], (11, 57)),
        ([(1, 12), (141, 1), (147, 130), (16, 150)], (11, 60)),
        ([(1, 15), (165, 1), (175, 135), (1, 171)], (-6, 46)),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("hit_screen/{i:02}.png"))?;
        if i < 6 || i >= 22 {
            return Ok(frame);
        }
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].resize_fit((140, 120), Fit::Cover);
        let (points, pos) = params[i - 6];
        canvas.draw_image(&image.perspective(&points), pos, None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 29,
            duration: 0.2,
        },
        FrameAlign::ExtendFirst,
    )
}

register_meme!(
    "hit_screen",
    hit_screen,
    min_images = 1,
    max_images = 1,
    keywords = &["打穿", "打穿屏幕"],
    date_created = local_date(2022, 9, 30),
    date_modified = local_date(2023, 2, 14),
);
