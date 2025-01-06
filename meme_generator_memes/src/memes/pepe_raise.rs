use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn pepe_raise(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let left_locs = [
        (107, 30),
        (107, 30),
        (95, 45),
        (80, 160),
        (80, 160),
        (70, 98),
    ];
    let right_locs = [
        (320, 145),
        (320, 145),
        (330, 130),
        (300, 50),
        (300, 50),
        (323, 80),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("pepe_raise/{i}.png"))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let left_img = images[0].circle().resize_exact((100, 100));
        let right_img = images[1].circle().resize_exact((100, 100));
        canvas.draw_image(&left_img, left_locs[i], None);
        canvas.draw_image(&right_img, right_locs[i], None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 6,
            duration: 0.06,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "pepe_raise",
    pepe_raise,
    min_images = 2,
    max_images = 2,
    keywords = &["佩佩举"],
    tags = MemeTags::arknights(),
    date_created = local_date(2024, 8, 18),
    date_modified = local_date(2024, 8, 18),
);
