use core::f32;

use rand::Rng;
use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    tools::{local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn shake_head(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let img = &images[0].image;
    let img_w = img.width();
    let img_h = img.height();
    let padding_w = img_w / 10;
    let padding_h = img_h / 10;
    let dw = padding_w.max(1) / 8;
    let dh = padding_h.max(1) / 8;
    let frame_w = img_w - padding_w * 2;
    let frame_h = img_h - padding_h * 2;
    let frame_num = 20;
    let dt = 2.0 * f32::consts::PI / frame_num as f32;

    let func = |i: usize, images: Vec<Image>| {
        let mut surface = new_surface((frame_w, frame_h));
        let canvas = surface.canvas();
        let img = &images[0];
        let mut rng = rand::thread_rng();
        let x = (padding_w as f32 * (-(i as f32) * dt).sin() - padding_w as f32
            + rng.gen_range(-1.0..1.0) * dw as f32)
            .round() as i32;
        let y = (padding_h as f32 * (-(i as f32) * dt).cos() - padding_h as f32
            + rng.gen_range(-1.0..1.0) * dh as f32)
            .round() as i32;
        canvas.draw_image(&img, (x, y), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num,
            duration: 0.02,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "shake_head",
    shake_head,
    min_images = 1,
    max_images = 1,
    keywords = &["晃脑"],
    date_created = local_date(2024, 10, 31),
    date_modified = local_date(2024, 10, 31),
);
