use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn bocchi_draft(images: Vec<DecodedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let params = [
        ([(54, 62), (353, 1), (379, 382), (1, 399)], (146, 173)),
        ([(54, 61), (349, 1), (379, 381), (1, 398)], (146, 174)),
        ([(54, 61), (349, 1), (379, 381), (1, 398)], (152, 174)),
        ([(54, 61), (335, 1), (379, 381), (1, 398)], (158, 167)),
        ([(54, 61), (335, 1), (370, 381), (1, 398)], (157, 149)),
        ([(41, 59), (321, 1), (357, 379), (1, 396)], (167, 108)),
        ([(41, 57), (315, 1), (357, 377), (1, 394)], (173, 69)),
        ([(41, 56), (309, 1), (353, 380), (1, 393)], (175, 43)),
        ([(41, 56), (314, 1), (353, 380), (1, 393)], (174, 30)),
        ([(41, 50), (312, 1), (348, 367), (1, 387)], (171, 18)),
        ([(35, 50), (306, 1), (342, 367), (1, 386)], (178, 14)),
    ];
    let idx = [
        0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10,
    ];

    let func = |i: usize, images: &Vec<Image>| {
        let frame = load_image(format!("bocchi_draft/{i:02}.png"))?;
        let (points, pos) = params[idx[i]];
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        let image = images[0].resize_fit((350, 400), Fit::Cover);
        let image = image.perspective(&points);
        canvas.draw_image(&image, pos, None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 23,
            duration: 0.08,
        },
        FrameAlign::ExtendLast,
    )
}

register_meme!(
    "bocchi_draft",
    bocchi_draft,
    min_images = 1,
    max_images = 1,
    keywords = &["波奇手稿"],
    tags = MemeTags::bocchi(),
    date_created = local_date(2022, 11, 29),
    date_modified = local_date(2023, 2, 14),
);