use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn throw_gif(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let params = [
        vec![(32, 32, 108, 36)],
        vec![(32, 32, 122, 36)],
        Vec::new(),
        vec![(123, 123, 19, 129)],
        vec![(185, 185, -50, 200), (33, 33, 289, 70)],
        vec![(32, 32, 280, 73)],
        vec![(35, 35, 259, 31)],
        vec![(175, 175, -50, 220)],
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("throw_gif/{i}.png"))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let img = images[0].circle();
        for (w, h, x, y) in &params[i] {
            let img = img.resize_exact((*w, *h));
            canvas.draw_image(&img, (*x, *y), None);
        }
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 8,
            duration: 0.1,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "throw_gif",
    throw_gif,
    min_images = 1,
    max_images = 1,
    keywords = &["抛", "掷"],
    date_created = local_date(2022, 3, 9),
    date_modified = local_date(2023, 2, 14),
);
