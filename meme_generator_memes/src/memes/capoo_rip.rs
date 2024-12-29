use skia_safe::{Color, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    encoder::{make_gif_or_combined_gif, GifInfo},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn capoo_rip(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let params1 = [
        ((61, 196), [(140, 68), (0, 59), (33, 0), (165, 8)]),
        ((63, 196), [(136, 68), (0, 59), (29, 0), (158, 13)]),
        ((62, 195), [(137, 72), (0, 58), (27, 0), (167, 11)]),
        ((95, 152), [(0, 8), (155, 0), (163, 107), (13, 112)]),
        ((108, 129), [(0, 6), (128, 0), (136, 113), (10, 117)]),
        ((84, 160), [(0, 6), (184, 0), (190, 90), (10, 97)]),
    ];
    let params2 = [
        (
            ((78, 158), [(0, 3), (86, 0), (97, 106), (16, 106)]),
            ((195, 156), [(0, 4), (82, 0), (85, 106), (15, 110)]),
        ),
        (
            ((89, 156), [(0, 0), (80, 0), (94, 100), (14, 100)]),
            ((192, 151), [(0, 7), (79, 3), (82, 107), (11, 112)]),
        ),
    ];
    let mut raw_frames = vec![];
    for i in 0..8 {
        raw_frames.push(load_image(format!("capoo_rip/{i}.png"))?);
    }
    let indexes = [0, 1, 2, 0, 1, 2, 0, 1, 2, 3, 4, 5, 6, 7, 7];

    let func = |i: usize, images: &Vec<Image>| {
        let index = indexes[i];
        let frame = &raw_frames[index];
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        if (0..6).contains(&index) {
            let (pos, points) = params1[index];
            let image = images[0].resize_fit((150, 100), Fit::Cover);
            let image = image.perspective(&points);
            canvas.draw_image(&image, pos, None);
        } else {
            let (params1, params2) = params2[index - 6];
            let (pos1, points1) = params1;
            let (pos2, points2) = params2;
            let image = images[0].resize_fit((150, 100), Fit::Cover);
            let image_left = image.crop(IRect::from_ltrb(0, 0, 75, 100));
            let image_right = image.crop(IRect::from_ltrb(75, 0, 150, 100));
            let image_left = image_left.perspective(&points1);
            let image_right = image_right.perspective(&points2);
            canvas.draw_image(&image_left, pos1, None);
            canvas.draw_image(&image_right, pos2, None);
        }
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 15,
            duration: 0.1,
        },
        None,
    )
}

register_meme! {
    "capoo_rip",
    capoo_rip,
    min_images = 1,
    max_images = 1,
    keywords = &["咖波撕"],
    tags = MemeTags::capoo(),
    date_created = local_date(2023, 4, 17),
    date_modified = local_date(2023, 4, 28),
}
