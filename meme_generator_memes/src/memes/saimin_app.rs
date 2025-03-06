use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn saimin_app(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: Vec<Image>| {
        let app = load_image(format!("saimin_app/{i:02}.png"))?;
        let app_w = app.width();
        let app_h = app.height();
        let img = &images[0];
        let img_w = images[0].width();
        let img_h = images[0].height();
        let (frame_w, frame_h) = if img_w > img_h {
            (app_h * img_w / img_h, app_h)
        } else {
            (app_w, app_w * img_h / img_w)
        };

        let img = img.resize_fit((frame_w, frame_h), Fit::Cover);
        let mut surface = img.to_surface();
        let canvas = surface.canvas();
        canvas.draw_image(&app, (0, frame_h - app_h), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 25,
            duration: 0.03,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "saimin_app",
    saimin_app,
    min_images = 1,
    max_images = 1,
    keywords = &["催眠app"],
    date_created = local_date(2024, 12, 10),
    date_modified = local_date(2024, 12, 10),
);
