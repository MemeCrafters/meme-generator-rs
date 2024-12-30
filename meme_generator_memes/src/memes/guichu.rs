use skia_safe::{IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{DecodedImage, MemeOptions},
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{local_date, new_surface},
};

use crate::register_meme;

#[derive(MemeOptions)]
struct Direction {
    /// 鬼畜对称方向
    #[option(short, long, default="left", choices=["left", "right", "top", "bottom"])]
    direction: Option<String>,

    /// 左
    #[option(long, short_aliases=['左'])]
    left: Option<bool>,

    /// 右
    #[option(long, short_aliases=['右'])]
    right: Option<bool>,

    /// 上
    #[option(long, short_aliases=['上'])]
    top: Option<bool>,

    /// 下
    #[option(long, short_aliases=['下'])]
    bottom: Option<bool>,
}

fn guichu(images: Vec<DecodedImage>, _: Vec<String>, options: Direction) -> Result<Vec<u8>, Error> {
    let direction = if options.left.unwrap_or(false) {
        "left"
    } else if options.right.unwrap_or(false) {
        "right"
    } else if options.top.unwrap_or(false) {
        "top"
    } else if options.bottom.unwrap_or(false) {
        "bottom"
    } else {
        options.direction.as_deref().unwrap()
    };

    let image = &images[0];
    let img_w = image.width();
    let img_h = image.height();

    let params = match direction {
        "left" => (
            IRect::from_ltrb(0, 0, img_w / 2, img_h),
            (0, 0),
            IRect::from_ltrb(img_w / 2, 0, img_w / 2 * 2, img_h),
            (img_w / 2, 0),
        ),
        "right" => (
            IRect::from_ltrb(img_w / 2, 0, img_w / 2 * 2, img_h),
            (img_w / 2, 0),
            IRect::from_ltrb(0, 0, img_w / 2, img_h),
            (0, 0),
        ),
        "top" => (
            IRect::from_ltrb(0, 0, img_w, img_h / 2),
            (0, 0),
            IRect::from_ltrb(0, img_h / 2, img_w, img_h / 2 * 2),
            (0, img_h / 2),
        ),
        "bottom" => (
            IRect::from_ltrb(0, img_h / 2, img_w, img_h / 2 * 2),
            (0, img_h / 2),
            IRect::from_ltrb(0, 0, img_w, img_h / 2),
            (0, 0),
        ),
        _ => unreachable!(),
    };

    let func = |i: usize, images: &Vec<Image>| {
        let image = &images[0];

        if [0, 1, 2, 6, 7, 8, 12, 13, 14, 18, 20, 22].contains(&i) {
            Ok(image.clone())
        } else {
            let image_flip = match direction {
                "left" => image.flip_horizontal(),
                "right" => image.flip_horizontal(),
                "top" => image.flip_vertical(),
                "bottom" => image.flip_vertical(),
                _ => unreachable!(),
            };

            if [3, 4, 5, 9, 10, 11, 15, 16, 17, 19, 21, 23].contains(&i) {
                Ok(image_flip)
            } else {
                let (size1, pos1, size2, pos2) = params;
                let mut surface = new_surface(image.dimensions());
                let canvas = surface.canvas();
                canvas.draw_image(&image.crop(size1), pos1, None);
                canvas.draw_image(&image_flip.crop(size2), pos2, None);
                let image_symmetric = surface.image_snapshot();

                if [24, 25, 28, 29].contains(&i) {
                    Ok(image_symmetric)
                } else {
                    let mut surface = new_surface(image.dimensions());
                    let canvas = surface.canvas();
                    let image_symmetric_big = image_symmetric.resize_width(img_w * 2);
                    canvas.draw_image(&image_symmetric_big, (-img_w / 2, -img_h / 2), None);
                    Ok(surface.image_snapshot())
                }
            }
        }
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 32,
            duration: 0.2,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "guichu",
    guichu,
    min_images = 1,
    max_images = 1,
    keywords = &["鬼畜"],
    date_created = local_date(2023, 7, 19),
    date_modified = local_date(2023, 7, 19),
);
