use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn tankuku_raisesign(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let params = [
        ([(0, 46), (320, 0), (350, 214), (38, 260)], (68, 91)),
        ([(18, 0), (328, 28), (298, 227), (0, 197)], (184, 77)),
        ([(15, 0), (294, 28), (278, 216), (0, 188)], (194, 65)),
        ([(14, 0), (279, 27), (262, 205), (0, 178)], (203, 55)),
        ([(14, 0), (270, 25), (252, 195), (0, 170)], (209, 49)),
        ([(15, 0), (260, 25), (242, 186), (0, 164)], (215, 41)),
        ([(10, 0), (245, 21), (230, 180), (0, 157)], (223, 35)),
        ([(13, 0), (230, 21), (218, 168), (0, 147)], (231, 25)),
        ([(13, 0), (220, 23), (210, 167), (0, 140)], (238, 21)),
        ([(27, 0), (226, 46), (196, 182), (0, 135)], (254, 13)),
        ([(27, 0), (226, 46), (196, 182), (0, 135)], (254, 13)),
        ([(27, 0), (226, 46), (196, 182), (0, 135)], (254, 13)),
        ([(0, 35), (200, 0), (224, 133), (25, 169)], (175, 9)),
        ([(0, 35), (200, 0), (224, 133), (25, 169)], (195, 17)),
        ([(0, 35), (200, 0), (224, 133), (25, 169)], (195, 17)),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("tankuku_raisesign/{i:02}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (points, pos) = params[i];
        let img = images[0]
            .resize_fit((300, 230), Fit::Cover)
            .perspective(&points);
        canvas.draw_image(&img, pos, None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 15,
            duration: 0.2,
        },
        FrameAlign::ExtendFirst,
    )
}

register_meme!(
    "tankuku_raisesign",
    tankuku_raisesign,
    min_images = 1,
    max_images = 1,
    tags = MemeTags::tan_kuku(),
    keywords = &["唐可可举牌"],
    date_created = local_date(2022, 10, 1),
    date_modified = local_date(2023, 2, 14),
);
