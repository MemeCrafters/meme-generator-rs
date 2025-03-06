use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn thump(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let params = [
        (65, 128, 77, 72),
        (67, 128, 73, 72),
        (54, 139, 94, 61),
        (57, 135, 86, 65),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("thump/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (x, y, w, h) = params[i];
        let img = images[0].square().resize_exact((w, h));
        canvas.draw_image(&img, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 4,
            duration: 0.04,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "thump",
    thump,
    min_images = 1,
    max_images = 1,
    keywords = &["捶"],
    date_created = local_date(2022, 3, 30),
    date_modified = local_date(2023, 2, 14),
);
