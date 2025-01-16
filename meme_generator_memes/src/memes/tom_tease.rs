use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn tom_tease(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("tom_tease/{i:02}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let img = images[0].resize_fit((400, 350), Fit::Cover);
        let img = img.perspective(&[(0, 100), (290, 0), (290, 370), (0, 335)]);
        canvas.draw_image(&img, (258, -12), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 11,
            duration: 0.2,
        },
        FrameAlign::ExtendFirst,
    )
}

register_meme!(
    "tom_tease",
    tom_tease,
    min_images = 1,
    max_images = 1,
    keywords = &["汤姆嘲笑"],
    tags = MemeTags::tom(),
    date_created = local_date(2024, 1, 19),
    date_modified = local_date(2024, 1, 19),
);
