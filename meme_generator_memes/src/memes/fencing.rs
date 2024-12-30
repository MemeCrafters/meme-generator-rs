use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn fencing(images: Vec<DecodedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let user_locs = [
        (57, 4),
        (55, 5),
        (58, 7),
        (57, 5),
        (53, 8),
        (54, 9),
        (64, 5),
        (66, 8),
        (70, 9),
        (73, 8),
        (81, 10),
        (77, 10),
        (72, 4),
        (79, 8),
        (50, 8),
        (60, 7),
        (67, 6),
        (60, 6),
        (50, 9),
    ];
    let self_locs = [
        (10, 6),
        (3, 6),
        (32, 7),
        (22, 7),
        (13, 4),
        (21, 6),
        (30, 6),
        (22, 2),
        (22, 3),
        (26, 8),
        (23, 8),
        (27, 10),
        (30, 9),
        (17, 6),
        (12, 8),
        (11, 7),
        (8, 6),
        (-2, 10),
        (4, 9),
    ];

    let func = |i: usize, images: &Vec<Image>| {
        let frame = load_image(format!("fencing/{i:02}.png"))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let self_head = images[0].circle().resize_exact((27, 27));
        let user_head = images[1].circle().resize_exact((27, 27));
        canvas.draw_image(&user_head, user_locs[i], None);
        canvas.draw_image(&self_head, self_locs[i], None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 19,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "fencing",
    fencing,
    min_images = 2,
    max_images = 2,
    keywords = &["击剑", "🤺"],
    date_created = local_date(2022, 10, 1),
    date_modified = local_date(2023, 2, 14),
);
