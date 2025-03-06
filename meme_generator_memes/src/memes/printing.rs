use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn printing(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("printing/{i:03}.png"))?;
        if (50..115).contains(&i) {
            let mut surface = new_surface(frame.dimensions());
            let canvas = surface.canvas();
            canvas.clear(Color::WHITE);
            let img = images[0].resize_bound((304, 174), Fit::Contain);
            canvas.draw_image(&img, (298 - img.width() / 2, 339 - img.height()), None);
            canvas.draw_image(&frame, (0, 0), None);
            Ok(surface.image_snapshot())
        } else {
            Ok(frame)
        }
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 115,
            duration: 0.05,
        },
        None,
    )
}

register_meme!(
    "printing",
    printing,
    min_images = 1,
    max_images = 1,
    keywords = &["打印"],
    date_created = local_date(2023, 1, 26),
    date_modified = local_date(2023, 2, 14),
);
