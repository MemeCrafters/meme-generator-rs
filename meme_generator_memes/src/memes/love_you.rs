use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn love_you(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [(68, 65, 70, 70), (63, 59, 80, 80)];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("love_you/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].square();
        let (x, y, w, h) = locs[i];
        let image = image.resize_exact((w, h));
        canvas.draw_image(&image, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 2,
            duration: 0.2,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "love_you",
    love_you,
    min_images = 1,
    max_images = 1,
    keywords = &["永远爱你"],
    date_created = local_date(2022, 3, 13),
    date_modified = local_date(2023, 2, 14),
);
