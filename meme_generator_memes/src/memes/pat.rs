use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn pat(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [(11, 73, 106, 100), (8, 79, 112, 96)];
    let indexes = [
        0, 1, 2, 3, 1, 2, 3, 0, 1, 2, 3, 0, 0, 1, 2, 3, 0, 0, 0, 0, 4, 5, 5, 5, 6, 7, 8, 9,
    ];

    let func = |i: usize, images: Vec<Image>| {
        let index = indexes[i];
        let frame = load_image(format!("pat/{index}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (x, y, w, h) = if index == 2 { locs[1] } else { locs[0] };
        let img = images[0].square().resize_exact((w, h));
        canvas.draw_image(&img, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 28,
            duration: 0.085,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "pat",
    pat,
    min_images = 1,
    max_images = 1,
    keywords = &["拍"],
    date_created = local_date(2021, 12, 1),
    date_modified = local_date(2023, 2, 14),
);
