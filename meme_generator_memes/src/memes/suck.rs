use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn suck(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (82, 100, 130, 119),
        (82, 94, 126, 125),
        (82, 120, 128, 99),
        (81, 164, 132, 55),
        (79, 163, 132, 55),
        (82, 140, 127, 79),
        (83, 152, 125, 67),
        (75, 157, 140, 62),
        (72, 165, 144, 54),
        (80, 132, 128, 87),
        (81, 127, 127, 92),
        (79, 111, 132, 108),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("suck/{i:02}.png"))?;
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
            frame_num: 12,
            duration: 0.08,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "suck",
    suck,
    min_images = 1,
    max_images = 1,
    keywords = &["吸", "嗦"],
    date_created = local_date(2022, 4, 20),
    date_modified = local_date(2023, 2, 14),
);
