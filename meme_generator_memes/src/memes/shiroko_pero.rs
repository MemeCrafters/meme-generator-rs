use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn shiroko_pero(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let mask = load_image("shiroko_pero/mask.png")?;

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("shiroko_pero/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        let img = images[0]
            .resize_fit((245, 245), Fit::Cover)
            .with_background(Color::WHITE)
            .clip_mask(&mask);
        canvas.draw_image(&img, (105, 178), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 4,
            duration: 0.06,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "shiroko_pero",
    shiroko_pero,
    min_images = 1,
    max_images = 1,
    keywords = &["白子舔"],
    tags = MemeTags::shiroko(),
    date_created = local_date(2024, 8, 10),
    date_modified = local_date(2024, 8, 10),
);
