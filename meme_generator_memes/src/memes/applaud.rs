use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn applaud(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (109, 102, 27, 17),
        (107, 105, 28, 15),
        (110, 106, 27, 14),
        (109, 106, 27, 14),
        (107, 108, 29, 12),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("applaud/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        let (w, h, x, y) = locs[i];
        let image = images[0].square().resize_exact((w, h));
        canvas.draw_image(&image, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 5,
            duration: 0.1,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "applaud",
    applaud,
    min_images = 1,
    max_images = 1,
    keywords = &["鼓掌"],
    date_created = local_date(2023, 1, 8),
    date_modified = local_date(2023, 2, 14),
);
