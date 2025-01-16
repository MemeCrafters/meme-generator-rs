use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn knock(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (60, 308, 210, 195),
        (60, 308, 210, 198),
        (45, 330, 250, 172),
        (58, 320, 218, 180),
        (60, 310, 215, 193),
        (40, 320, 250, 285),
        (48, 308, 226, 192),
        (51, 301, 223, 200),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("knock/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (x, y, w, h) = locs[i];
        let image = images[0].square().resize_exact((w, h));
        canvas.draw_image(&image, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 8,
            duration: 0.04,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "knock",
    knock,
    min_images = 1,
    max_images = 1,
    keywords = &["敲"],
    tags = MemeTags::gura(),
    date_created = local_date(2022, 4, 14),
    date_modified = local_date(2023, 2, 14),
);
