use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    tools::{local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn vibrate(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: Vec<Image>| {
        let img = &images[0];
        let w = img.width();
        let h = img.height();
        let locs = [
            (0, 0),
            (w / 25, h / 25),
            (w / 50, h / 50),
            (0, h / 25),
            (w / 25, 0),
        ];
        let mut surface = new_surface((w + w / 25, h + h / 25));
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        canvas.draw_image(&img, locs[i], None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 5,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "vibrate",
    vibrate,
    min_images = 1,
    max_images = 1,
    keywords = &["震动"],
    date_created = local_date(2023, 8, 28),
    date_modified = local_date(2023, 8, 28),
);
