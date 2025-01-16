use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn mahiro_readbook(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let positions = [(0, 118), (0, 117), (0, 116), (0, 116), (-3, 116), (-7, 117)];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("mahiro_readbook/{i:02}.png"))?;
        let mut surface = new_surface((240, 240));
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let img = images[0].resize_fit((70, 100), Fit::Cover);
        let img = img.perspective(&[(0, 6), (77, -5), (100, 100), (32, 100)]);
        let idx = (i - 16).max(0).min(5) as usize;
        canvas.draw_image(&img, positions[idx], None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 48,
            duration: 0.08,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "mahiro_readbook",
    mahiro_readbook,
    min_images = 1,
    max_images = 1,
    keywords = &["真寻看书"],
    tags = MemeTags::mahiro(),
    date_created = local_date(2024, 8, 18),
    date_modified = local_date(2024, 8, 18),
);
