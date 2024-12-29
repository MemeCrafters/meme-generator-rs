use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn hug(images: &mut Vec<DecodedImage>, _: &Vec<String>, _: &NoOptions) -> Result<Vec<u8>, Error> {
    let user_locs = [
        (108, 15),
        (107, 14),
        (104, 16),
        (102, 14),
        (104, 15),
        (108, 15),
        (108, 15),
        (103, 16),
        (102, 15),
        (104, 14),
    ];
    let self_locs = [
        (78, 120),
        (115, 130),
        (0, 0),
        (110, 100),
        (80, 100),
        (75, 115),
        (105, 127),
        (0, 0),
        (110, 98),
        (80, 105),
    ];
    let angles = [48, 18, 0, -38, -31, 43, 22, 0, -34, -35];

    let func = |i: usize, images: &Vec<Image>| {
        let frame = load_image(format!("hug/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let self_head = images[0].square().resize_exact((120, 120));
        let user_head = images[1].square().resize_exact((105, 105));
        canvas.draw_image(&user_head, user_locs[i], None);
        canvas.draw_image(&self_head.rotate(angles[i] as f32), self_locs[i], None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 10,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "hug",
    hug,
    min_images = 2,
    max_images = 2,
    keywords = &["抱", "抱抱"],
    date_created = local_date(2024, 8, 6),
    date_modified = local_date(2024, 8, 6),
);
