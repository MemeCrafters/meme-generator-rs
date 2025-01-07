use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn step_on(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let params = [
        (104, 72, 32, 185, 25),
        (104, 72, 32, 185, 25),
        (90, 73, 51, 207, 0),
        (88, 78, 51, 202, 0),
        (88, 86, 49, 197, 0),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("step_on/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (w, h, x, y, angle) = params[i];
        let img = images[0].square().resize_exact((w, h)).rotate(angle as f32);
        canvas.draw_image(&img, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 5,
            duration: 0.07,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "step_on",
    step_on,
    min_images = 1,
    max_images = 1,
    keywords = &["踩"],
    date_created = local_date(2023, 3, 28),
    date_modified = local_date(2023, 3, 28),
);
