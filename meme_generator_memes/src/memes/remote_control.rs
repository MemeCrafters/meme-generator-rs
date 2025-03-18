use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn remote_control(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let img = &images[0].image;
    let img_w = img.width();
    let img_h = img.height();
    let locs = vec![
        (0, 0),
        (img_w / 80, img_h / 80),
        (-img_w / 100, -img_h / 100),
        (img_w / 60, 0),
        (0, img_h / 60),
    ];
    let overlay = load_image("remote_control/0.png")?;
    let overlay = overlay.resize_height((img_h as f32 / 2.5) as i32);

    let func = |i: usize, images: Vec<Image>| {
        let mut surface = new_surface((img_w, img_h));
        let canvas = surface.canvas();
        canvas.draw_image(&images[0], locs[i], None);
        let x = img_w - overlay.width();
        let y = img_h - overlay.height();
        canvas.draw_image(&overlay, (x, y), None);
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
    "remote_control",
    remote_control,
    min_images = 1,
    max_images = 1,
    keywords = &["遥控", "控制"],
    date_created = local_date(2025, 3, 4),
    date_modified = local_date(2025, 3, 4),
);
