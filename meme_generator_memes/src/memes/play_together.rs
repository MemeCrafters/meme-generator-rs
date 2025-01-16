use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn play_together(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let params = vec![(194, 204, 121, 108); 36]
        .into_iter()
        .chain(vec![
            (192, 203, 125, 111),
            (182, 200, 141, 126),
            (161, 188, 178, 159),
            (129, 171, 235, 209),
            (98, 155, 290, 258),
            (98, 155, 290, 258),
            (58, 133, 361, 321),
            (45, 126, 384, 342),
            (45, 126, 384, 342),
            (35, 121, 402, 358),
            (27, 117, 415, 370),
            (17, 111, 433, 386),
            (14, 110, 439, 391),
            (14, 110, 439, 391),
            (11, 108, 444, 395),
            (10, 108, 446, 397),
        ])
        .chain(vec![(7, 106, 451, 402); 8])
        .collect::<Vec<_>>();

    let func = |i: usize, images: Vec<Image>| {
        let (x, y, w, h) = params[i];
        let screen = images[0].resize_exact((w, h));
        let frame = load_image(format!("play_together/{i:02}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        canvas.draw_image(&screen, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 60,
            duration: 0.06,
        },
        FrameAlign::ExtendLast,
    )
}

register_meme!(
    "play_together",
    play_together,
    min_images = 1,
    max_images = 1,
    keywords = &["一起玩"],
    tags = MemeTags::blue_archive(),
    date_created = local_date(2024, 7, 26),
    date_modified = local_date(2024, 7, 26),
);
