use core::f32;

use skia_safe::{ClipOp, Image, Path, Point};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn crop_to_triangle(img: &Image) -> Image {
    let img_w = img.width() as f32;
    let img_h = img.height() as f32;

    let path = Path::polygon(
        &[
            Point::new(img_w / 2.0, 0.0),
            Point::new(0.0, img_h),
            Point::new(img_w, img_h),
        ],
        true,
        None,
        None,
    );
    img.clip_path(&path, ClipOp::Intersect)
}

fn pyramid(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let img_w = 300;
    let img_h = 300;
    let fov = 45.0 * f32::consts::PI / 180.0;
    let z = (img_w as f32).hypot(img_h as f32) / (2.0 * (fov / 2.0).tan());
    let a: f32 = 180.0;
    let h: f32 = 180.0;
    let rh = a.hypot(h);

    let rotate_y = |img: &Image, theta: f32| -> ((f32, f32), Image) {
        let orgs = [
            (-a / 2.0, -h / 2.0, 0.0),
            (a / 2.0, -h / 2.0, 0.0),
            (a / 2.0, h / 2.0, a / 2.0),
            (-a / 2.0, h / 2.0, a / 2.0),
        ];
        let cos = theta.cos();
        let sin = theta.sin();

        let points = orgs
            .iter()
            .map(|org| {
                let dst = (org.0 * cos + org.2 * sin, org.1, -org.0 * sin + org.2 * cos);
                (dst.0 * z / (z - dst.2), dst.1 * z / (z - dst.2))
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

        let pos = (img_w as f32 / 2.0 + min_x, img_h as f32 / 2.0 + min_y);

        let img = img.resize_fit((a as i32, rh as i32), Fit::Cover);
        let img = crop_to_triangle(&img);
        let img = img.perspective(&points);
        (pos, img)
    };

    let frame_num_per_image = 15;
    let frame_num = images.len() * frame_num_per_image;
    let theta_step = 90.0 / frame_num_per_image as f32;

    let func = |i: usize, images: Vec<Image>| {
        let img_idx1 = i / frame_num_per_image;
        let img_idx2 = (img_idx1 + 1) % images.len();
        let theta1 = (i % frame_num_per_image) as f32 * theta_step;
        let theta2 = theta1 - 90.0;

        let mut surface = new_surface((img_w, img_h));
        let canvas = surface.canvas();
        let (pos1, img1) = rotate_y(&images[img_idx1], theta1.to_radians());
        let (pos2, img2) = rotate_y(&images[img_idx2], theta2.to_radians());
        canvas.draw_image(&img2, pos2, None);
        canvas.draw_image(&img1, pos1, None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: frame_num as u32,
            duration: 0.06,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "pyramid",
    pyramid,
    min_images = 1,
    max_images = 4,
    keywords = &["四棱锥", "金字塔"],
    date_created = local_date(2024, 8, 16),
    date_modified = local_date(2024, 8, 18),
);
