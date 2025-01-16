use skia_safe::{IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn windmill_turn(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: Vec<Image>| {
        let mut surface = new_surface((600, 600));
        let canvas = surface.canvas();
        let img = images[0].square().resize_exact((300, 300));
        canvas.draw_image(&img, (0, 0), None);
        canvas.draw_image(&img.rotate(90.0), (0, 300), None);
        canvas.draw_image(&img.rotate(180.0), (300, 300), None);
        canvas.draw_image(&img.rotate(270.0), (300, 0), None);
        let frame = surface.image_snapshot();
        Ok(frame
            .rotate_crop(i as f32 * 18.0)
            .crop(IRect::from_ltrb(50, 50, 550, 550)))
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 5,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "windmill_turn",
    windmill_turn,
    min_images = 1,
    max_images = 1,
    keywords = &["风车转"],
    date_created = local_date(2022, 12, 13),
    date_modified = local_date(2023, 2, 14),
);
