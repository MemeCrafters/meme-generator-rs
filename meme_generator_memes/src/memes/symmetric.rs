use skia_safe::{IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{local_date, new_surface},
};

use crate::register_meme;

#[derive(MemeOptions)]
struct Direction {
    /// 对称方向
    #[option(short, long, default="left", choices=["left", "right", "top", "bottom"])]
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

fn symmetric(
    images: Vec<InputImage>,
    _: Vec<String>,
    options: Direction,
) -> Result<Vec<u8>, Error> {
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

    let img = &images[0].image;
    let img_w = img.width();
    let img_h = img.height();

    let (frame_size, size1, pos1, size2, pos2) = match direction {
        "left" => (
            (img_w / 2 * 2, img_h),
            IRect::from_ltrb(0, 0, img_w / 2, img_h),
            (0, 0),
            IRect::from_ltrb(img_w / 2, 0, img_w / 2 * 2, img_h),
            (img_w / 2, 0),
        ),
        "right" => (
            (img_w / 2 * 2, img_h),
            IRect::from_ltrb(img_w / 2, 0, img_w / 2 * 2, img_h),
            (img_w / 2, 0),
            IRect::from_ltrb(0, 0, img_w / 2, img_h),
            (0, 0),
        ),
        "top" => (
            (img_w, img_h / 2 * 2),
            IRect::from_ltrb(0, 0, img_w, img_h / 2),
            (0, 0),
            IRect::from_ltrb(0, img_h / 2, img_w, img_h / 2 * 2),
            (0, img_h / 2),
        ),
        "bottom" => (
            (img_w, img_h / 2 * 2),
            IRect::from_ltrb(0, img_h / 2, img_w, img_h / 2 * 2),
            (0, img_h / 2),
            IRect::from_ltrb(0, 0, img_w, img_h / 2),
            (0, 0),
        ),
        _ => unreachable!(),
    };

    let func = |images: Vec<Image>| {
        let img = &images[0];
        let first = img.clone();
        let second = match direction {
            "left" => img.flip_horizontal(),
            "right" => img.flip_horizontal(),
            "top" => img.flip_vertical(),
            "bottom" => img.flip_vertical(),
            _ => unreachable!(),
        };

        let mut surface = new_surface(frame_size);
        let canvas = surface.canvas();
        canvas.draw_image(&first.crop(size1), pos1, None);
        canvas.draw_image(&second.crop(size2), pos2, None);

        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme! {
    "symmetric",
    symmetric,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["对称"],
    date_created = local_date(2022, 3, 14),
    date_modified = local_date(2023, 2, 14),
}
