use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};
fn hug_leg(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (50, 73, 68, 92),
        (58, 60, 62, 95),
        (65, 10, 67, 118),
        (61, 20, 77, 97),
        (55, 44, 65, 106),
        (66, 85, 60, 98),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("hug_leg/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (x, y, w, h) = locs[i];
        let image = images[0].square().resize_exact((w, h));
        canvas.draw_image(&image, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
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
    "hug_leg",
    hug_leg,
    min_images = 1,
    max_images = 1,
    keywords = &["抱大腿"],
    date_created = local_date(2022, 10, 1),
    date_modified = local_date(2023, 2, 14),
);
