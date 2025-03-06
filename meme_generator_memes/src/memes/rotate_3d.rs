use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn rotate_y(img: &Image, theta: f32, z: f32) -> Image {
    let pc = (img.width() as f32 / 2.0, img.height() as f32 / 2.0, 0.0);
    let orgs = [
        (0.0, 0.0, 0.0),
        (img.width() as f32, 0.0, 0.0),
        (img.width() as f32, img.height() as f32, 0.0),
        (0.0, img.height() as f32, 0.0),
    ];
    let cos = theta.cos();
    let sin = theta.sin();

    let points = orgs
        .iter()
        .map(|org| {
            let dst = (
                (org.0 - pc.0) * cos + (org.2 - pc.2) * sin,
                org.1 - pc.1,
                -(org.0 - pc.0) * sin + (org.2 - pc.2) * cos,
            );
            (
                dst.0 * z / (z - dst.2) + pc.0,
                dst.1 * z / (z - dst.2) + pc.1,
            )
        })
        .collect::<Vec<_>>();

    let min_x = points
        .iter()
        .map(|point| point.0)
        .fold(f32::INFINITY, f32::min);
    let min_y = points
        .iter()
        .map(|point| point.1)
        .fold(f32::INFINITY, f32::min);
    let points: [(f32, f32); 4] = points
        .into_iter()
        .map(|dst| (dst.0 - min_x, dst.1 - min_y))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    img.perspective(&points)
}

fn rotate_3d(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let img = &images[0].image;
    let img_w = img.width();
    let img_h = img.height();

    let fov = 45_f32.to_radians();
    let z = (img_w as f32).hypot(img_h as f32) / (2.0 * (fov / 2.0).tan());
    let frame_w = (img_w as f32 * 1.2).round() as i32;
    let frame_h = (img_h as f32 * 1.5).round() as i32;

    let func = |i: usize, images: Vec<Image>| {
        let mut surface = new_surface((frame_w, frame_h));
        let canvas = surface.canvas();
        let img = &images[0];
        let rotated = rotate_y(&img, i as f32 * 12_f32.to_radians(), z);
        canvas.draw_image(
            &rotated,
            (
                (frame_w - rotated.width()) / 2,
                (frame_h - rotated.height()) / 2,
            ),
            None,
        );
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 30,
            duration: 0.07,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "rotate_3d",
    rotate_3d,
    min_images = 1,
    max_images = 1,
    keywords = &["三维旋转"],
    date_created = local_date(2024, 4, 30),
    date_modified = local_date(2024, 4, 30),
);
