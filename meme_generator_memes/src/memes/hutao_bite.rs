use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn hutao_bite(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [(98, 101, 108, 234), (96, 100, 108, 237)];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("hutao_bite/{i}.png"))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
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
            frame_num: 2,
            duration: 0.1,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "hutao_bite",
    hutao_bite,
    min_images = 1,
    max_images = 1,
    keywords = &["胡桃啃"],
    tags = MemeTags::hutao(),
    date_created = local_date(2022, 11, 29),
    date_modified = local_date(2023, 2, 14),
);
