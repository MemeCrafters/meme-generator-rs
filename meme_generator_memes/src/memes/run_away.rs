use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn run_away(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: Vec<Image>| {
        let miku = load_image(format!("run_away/{i:02}.png"))?;
        let miku_w = miku.width();
        let miku_h = miku.height();
        let img = &images[0];
        let img_w = img.width();
        let img_h = img.height();
        let ratio = 1.2;
        let (frame_w, frame_h) = if img_w > img_h {
            let frame_h = (miku_h as f32 * ratio).round() as i32;
            let frame_w = (frame_h as f32 * img_w as f32 / img_h as f32).round() as i32;
            (frame_w, frame_h)
        } else {
            let frame_w = (miku_w as f32 * ratio).round() as i32;
            let frame_h = (frame_w as f32 * img_h as f32 / img_w as f32).round() as i32;
            (frame_w, frame_h)
        };

        let img = img.resize_fit((frame_w, frame_h), Fit::Cover);
        let mut surface = img.to_surface();
        let canvas = surface.canvas();
        canvas.draw_image(&miku, (frame_w - miku_w, frame_h - miku_h), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 42,
            duration: 0.03,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "run_away",
    run_away,
    min_images = 1,
    max_images = 1,
    keywords = &["快逃"],
    tags = MemeTags::miku(),
    date_created = local_date(2024, 7, 23),
    date_modified = local_date(2024, 7, 23),
);
