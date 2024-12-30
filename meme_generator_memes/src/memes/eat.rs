use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn eat(images: Vec<DecodedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: &Vec<Image>| {
        let frame = load_image(format!("eat/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].square().resize_exact((34, 34));
        canvas.draw_image(&image, (2, 38), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 3,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "eat",
    eat,
    min_images = 1,
    max_images = 1,
    keywords = &["吃"],
    date_created = local_date(2022, 2, 15),
    date_modified = local_date(2023, 2, 14),
);
