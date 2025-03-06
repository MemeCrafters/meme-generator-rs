use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn nahida_bite(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (123, 356, 158, 124),
        (123, 356, 158, 124),
        (123, 355, 158, 125),
        (122, 352, 159, 128),
        (122, 350, 159, 130),
        (122, 348, 159, 132),
        (122, 345, 159, 135),
        (121, 343, 160, 137),
        (121, 342, 160, 138),
        (121, 341, 160, 139),
        (121, 341, 160, 139),
        (121, 342, 160, 138),
        (121, 344, 160, 136),
        (121, 346, 160, 134),
        (122, 349, 159, 131),
        (122, 351, 159, 129),
        (122, 353, 159, 127),
        (123, 355, 158, 125),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("nahida_bite/{i:02}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let img = images[0].resize_fit((160, 140), Fit::Cover);
        let (x, y, w, h) = locs[i % locs.len()];
        let img = img.resize_exact((w, h));
        canvas.draw_image(&img, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 38,
            duration: 0.1,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "nahida_bite",
    nahida_bite,
    min_images = 1,
    max_images = 1,
    keywords = &["纳西妲啃", "草神啃"],
    tags = MemeTags::nahida(),
    date_created = local_date(2023, 6, 23),
    date_modified = local_date(2024, 8, 10),
);
