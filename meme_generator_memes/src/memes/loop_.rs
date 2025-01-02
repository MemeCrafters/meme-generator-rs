use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{MemeOptions, NamedImage},
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    tools::{local_date, new_surface},
};

use crate::register_meme;

#[derive(MemeOptions)]
struct Direction {
    /// 循环方向
    #[option(short, long, default="top", choices=["left", "right", "top", "bottom"])]
    direction: Option<String>,

    /// 左
    #[option(long, short_aliases=['左'], default=false)]
    left: Option<bool>,

    /// 右
    #[option(long, short_aliases=['右'], default=false)]
    right: Option<bool>,

    /// 上
    #[option(long, short_aliases=['上'], default=false)]
    top: Option<bool>,

    /// 下
    #[option(long, short_aliases=['下'], default=false)]
    bottom: Option<bool>,
}

fn loop_(images: Vec<NamedImage>, _: Vec<String>, options: Direction) -> Result<Vec<u8>, Error> {
    let direction = if options.left.unwrap() {
        "left"
    } else if options.right.unwrap() {
        "right"
    } else if options.top.unwrap() {
        "top"
    } else if options.bottom.unwrap() {
        "bottom"
    } else {
        options.direction.as_deref().unwrap()
    };

    let func = |i: usize, images: Vec<Image>| {
        let img = &images[0];
        let img_w = img.width();
        let img_h = img.height();
        let mut surface = new_surface((img_w, img_h));
        let canvas = surface.canvas();

        if direction == "top" || direction == "bottom" {
            let h = ((img_h as f32 / 30.0) * i as f32) as i32;
            if direction == "top" {
                canvas.draw_image(&img, (0, -h), None);
                canvas.draw_image(&img, (0, img_h - h), None);
            } else {
                canvas.draw_image(&img, (0, h), None);
                canvas.draw_image(&img, (0, h - img_h), None);
            }
        } else {
            let w = ((img_w as f32 / 30.0) * i as f32) as i32;
            if direction == "left" {
                canvas.draw_image(&img, (-w, 0), None);
                canvas.draw_image(&img, (img_w - w, 0), None);
            } else {
                canvas.draw_image(&img, (w, 0), None);
                canvas.draw_image(&img, (w - img_w, 0), None);
            }
        };

        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 30,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "loop",
    loop_,
    min_images = 1,
    max_images = 1,
    keywords = &["循环"],
    date_created = local_date(2024, 7, 14),
    date_modified = local_date(2024, 8, 15),
);
