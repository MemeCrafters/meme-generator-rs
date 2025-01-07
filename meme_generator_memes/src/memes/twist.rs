use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn twist(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let params = [
        (25, 66, 0),
        (25, 66, 60),
        (23, 68, 120),
        (20, 69, 180),
        (22, 68, 240),
        (25, 66, 300),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("twist/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (x, y, a) = params[i];
        let img = images[0]
            .square()
            .rotate_crop(a as f32)
            .resize_exact((78, 78));
        canvas.draw_image(&img, (x, y), None);
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
    "twist",
    twist,
    min_images = 1,
    max_images = 1,
    keywords = &["搓"],
    date_created = local_date(2022, 3, 9),
    date_modified = local_date(2023, 2, 14),
);
