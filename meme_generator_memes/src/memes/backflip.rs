use core::f32;
use skia_safe::{Image, Matrix};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    tools::{local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn backflip(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let img = &images[0].image;
    let img_w = img.width();
    let img_h = img.height();

    let length = ((img_w * img_w + img_h * img_h) as f32).sqrt();
    let frame_w = (length * 1.3) as i32;
    let bounce_h = img_h as f32 * 1.2;
    let frame_h = (bounce_h + length / 2.0 + img_h as f32 * 0.6) as i32;
    let center_x = (frame_w / 2) as f32;
    let ground_y = (frame_h - img_h / 2) as f32;

    let total_frames = 30;
    let bounce1_range = 0.0..0.3;
    let bounce2_range = 0.3..0.6;
    let rise_range = 0.6..0.65;
    let flip_range = 0.65..0.95;
    let land_range = 0.95..1.0;

    let func = |i: usize, images: Vec<Image>| {
        let t = i as f32 / total_frames as f32;

        let (y, angle) = if bounce1_range.contains(&t) {
            let local_t = (t - bounce1_range.start) / (bounce1_range.end - bounce1_range.start);
            let y = -4.0 * bounce_h * (local_t - 0.5).powi(2) + bounce_h;
            let rot = 45.0 * (1.0 - 2.0 * (local_t - 0.5).abs());
            (ground_y - y, rot)
        } else if bounce2_range.contains(&t) {
            let local_t = (t - bounce2_range.start) / (bounce2_range.end - bounce2_range.start);
            let y = -4.0 * bounce_h * (local_t - 0.5).powi(2) + bounce_h;
            let rot = -45.0 * (1.0 - 2.0 * (local_t - 0.5).abs());
            (ground_y - y, rot)
        } else if rise_range.contains(&t) {
            let local_t = (t - rise_range.start) / (rise_range.end - rise_range.start);
            let y = bounce_h - bounce_h * (1.0 - local_t).powi(2);
            (ground_y - y, 0.0)
        } else if flip_range.contains(&t) {
            let local_t = (t - flip_range.start) / (flip_range.end - flip_range.start);
            let y = bounce_h;
            let rot = 360.0 * local_t;
            (ground_y - y, rot)
        } else if land_range.contains(&t) {
            let local_t = (t - land_range.start) / (land_range.end - land_range.start);
            let y = bounce_h - bounce_h * local_t.powi(2);
            (ground_y - y, 0.0)
        } else {
            (ground_y, 0.0)
        };

        let img = &images[0];
        let mut surface = new_surface((frame_w, frame_h));
        let canvas = surface.canvas();
        let mut matrix = Matrix::new_identity();
        matrix.pre_translate((center_x, y));
        matrix.pre_rotate(angle, None);
        matrix.pre_translate((-img_w as f32 / 2.0, -img_h as f32 / 2.0));
        canvas.concat(&matrix);
        canvas.draw_image(img, (0, 0), None);

        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: total_frames,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "backflip",
    backflip,
    min_images = 1,
    max_images = 1,
    keywords = &["后空翻"],
    date_created = local_date(2025, 6, 29),
    date_modified = local_date(2025, 6, 29),
);
