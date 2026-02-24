use rand::RngExt;
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
    let func = |i: usize, images: Vec<Image>| {
        let img = &images[0];
        let img_w = img.width().min(500);
        let img = img.resize_width(img_w);
        let img_w = img.width();
        let img_h = img.height();
        let mut surface = new_surface((img_w, img_h));
        let canvas = surface.canvas();
        let mut rng = rand::rng();
        let pos = if i < 4 {
            (0, 0)
        } else {
            let dx = (img_w as f32 * rng.random_range(-1.0..1.0) / 60.0) as i32;
            let dy = (img_h as f32 * rng.random_range(-1.0..1.0) / 60.0) as i32;
            (dx, dy)
        };
        canvas.draw_image(&img, pos, None);
        let overlay = load_image(format!("remote_control/{i:02}.png"))?;
        let overlay = overlay.resize_height((img_h as f32 / 1.5) as i32);
        let x = img_w - overlay.width();
        let y = img_h - overlay.height();
        canvas.draw_image(&overlay, (x, y), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 17,
            duration: 0.07,
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
    date_modified = local_date(2025, 3, 24),
);
