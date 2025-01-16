use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{local_date, new_surface},
};

use crate::register_meme;

#[derive(MemeOptions)]
struct Direction {
    /// 跑动方向
    #[option(short, long, default="left_right", choices=["left_right", "right_left"])]
    direction: Option<String>,

    /// 左右
    #[option(long, long_aliases=["左右"], default=false)]
    left_right: Option<bool>,

    /// 右左
    #[option(long, long_aliases=["右左"], default=false)]
    right_left: Option<bool>,
}

fn left_right_jump(
    images: Vec<InputImage>,
    _: Vec<String>,
    options: Direction,
) -> Result<Vec<u8>, Error> {
    let direction = if options.left_right.unwrap() {
        "left_right"
    } else if options.right_left.unwrap() {
        "right_left"
    } else {
        &options.direction.unwrap()
    };

    let img = &images[0].image;
    let img_w = 100;
    let img_h = img.height() * img_w / img.width();
    let frame_w = 300;
    let frame_h = img_h + 30;

    let traj = |x: f32| {
        let h = 15.0;
        let w = (frame_w - img_w) as f32 / 4.0;
        let k = h / w.powi(2);
        if x < img_w as f32 / 2.0 || x > frame_w as f32 - img_w as f32 / 2.0 {
            0.0
        } else if x < frame_w as f32 / 2.0 {
            h - k * (x - img_w as f32 / 2.0 - w).powi(2)
        } else {
            h - k * (x - img_w as f32 / 2.0 - w * 3.0).powi(2)
        }
    };

    let frame_num: u32 = 30;
    let dx = (frame_w - img_w) as f32 / (frame_num as f32 / 2.0 - 1.0);

    let func = |i: usize, images: Vec<Image>| {
        let mut img = images[0].resize_width(img_w);
        let x;
        if direction == "left_right" {
            if i >= (frame_num as usize / 2) {
                x = frame_w as f32 - img_w as f32 - dx * (frame_num as f32 - i as f32 - 1.0);
                img = img.flip_horizontal();
            } else {
                x = frame_w as f32 - img_w as f32 - dx * i as f32;
            }
        } else {
            if i >= (frame_num as usize / 2) {
                x = dx * (frame_num as f32 - i as f32 - 1.0);
                img = img.flip_horizontal();
            } else {
                x = dx * i as f32;
            }
        }
        let y = frame_h as f32 - (traj(x + img_w as f32 / 2.0) + img_h as f32);

        let mut surface = new_surface((frame_w, frame_h));
        let canvas = surface.canvas();
        canvas.draw_image(&img, (x.round() as i32, y.round() as i32), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num,
            duration: 0.04,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "left_right_jump",
    left_right_jump,
    min_images = 1,
    max_images = 1,
    keywords = &["左右横跳"],
    date_created = local_date(2024, 7, 14),
    date_modified = local_date(2024, 7, 14),
);
