use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn pound(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (135, 240, 138, 47),
        (135, 240, 138, 47),
        (150, 190, 105, 95),
        (150, 190, 105, 95),
        (148, 188, 106, 98),
        (146, 196, 110, 88),
        (145, 223, 112, 61),
        (145, 223, 112, 61),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("pound/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (x, y, w, h) = locs[i];
        let img = images[0].square().resize_exact((w, h));
        canvas.draw_image(&img, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 8,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "pound",
    pound,
    min_images = 1,
    max_images = 1,
    keywords = &["捣"],
    date_created = local_date(2022, 3, 30),
    date_modified = local_date(2023, 2, 14),
);
