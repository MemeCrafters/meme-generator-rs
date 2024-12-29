use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn jump(images: &mut Vec<DecodedImage>, _: &Vec<String>, _: &NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (15, 50),
        (13, 43),
        (15, 23),
        (14, 4),
        (16, -3),
        (16, -4),
        (14, 4),
        (15, 31),
    ];

    let func = |i: usize, images: &Vec<Image>| {
        let frame = load_image(format!("jump/{i}.png"))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image = images[0].circle().resize_exact((40, 40));
        canvas.draw_image(&image, locs[i], None);
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
    "jump",
    jump,
    min_images = 1,
    max_images = 1,
    keywords = &["跳"],
    date_created = local_date(2024, 7, 14),
    date_modified = local_date(2024, 7, 14),
);
