use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn rip_clothes(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: Vec<Image>| {
        let img = images[0].resize_fit((480, 270), Fit::Cover);
        if i <= 15 {
            let frame = load_image(format!("rip_clothes/{i:02}.png"))?;
            let mut surface = new_surface(frame.dimensions());
            let canvas = surface.canvas();
            canvas.clear(Color::WHITE);
            canvas.draw_image(&img, (0, 0), None);
            canvas.draw_image(&frame, (0, 0), None);
            Ok(surface.image_snapshot())
        } else {
            Ok(img)
        }
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 20,
            duration: 0.1,
        },
        FrameAlign::ExtendLast,
    )
}

register_meme!(
    "rip_clothes",
    rip_clothes,
    min_images = 1,
    max_images = 1,
    keywords = &["撕衣服"],
    date_created = local_date(2025, 5, 7),
    date_modified = local_date(2025, 6, 3),
);
