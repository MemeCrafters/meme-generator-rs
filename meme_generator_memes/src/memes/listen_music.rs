use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn listen_music(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("listen_music/0.png")?;

    let func = |i: usize, images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0]
            .square()
            .resize_exact((215, 215))
            .rotate_crop(i as f32 * 10.0);
        canvas.draw_image(&image, (100, 100), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 36,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "listen_music",
    listen_music,
    min_images = 1,
    max_images = 1,
    keywords = &["听音乐"],
    date_created = local_date(2022, 3, 12),
    date_modified = local_date(2023, 2, 14),
);
