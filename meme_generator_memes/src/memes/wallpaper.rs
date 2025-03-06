use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn wallpaper(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("wallpaper/{i:02}.png"))?;

        if i < 8 {
            Ok(frame)
        } else {
            let mut surface = new_surface(frame.dimensions());
            let canvas = surface.canvas();
            canvas.clear(Color::WHITE);
            let img = images[0].resize_fit((515, 383), Fit::Cover);
            canvas.draw_image(&img, (176, -9), None);
            canvas.draw_image(&frame, (0, 0), None);
            Ok(surface.image_snapshot())
        }
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 20,
            duration: 0.07,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "wallpaper",
    wallpaper,
    min_images = 1,
    max_images = 1,
    keywords = &["墙纸"],
    tags = MemeTags::rick(),
    date_created = local_date(2022, 3, 9),
    date_modified = local_date(2023, 2, 14),
);
