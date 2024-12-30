use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn kick_ball(images: Vec<DecodedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (57, 136),
        (56, 117),
        (55, 99),
        (52, 113),
        (50, 126),
        (48, 139),
        (47, 112),
        (47, 85),
        (47, 57),
        (48, 97),
        (50, 136),
        (51, 176),
        (52, 169),
        (55, 181),
        (58, 153),
    ];

    let func = |i: usize, images: &Vec<Image>| {
        let frame = load_image(format!("kick_ball/{i:02}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].square().resize_exact((78, 78));
        let image = image.rotate_crop(24.0 * i as f32);
        canvas.draw_image(&image, locs[i], None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 15,
            duration: 0.1,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "kick_ball",
    kick_ball,
    min_images = 1,
    max_images = 1,
    keywords = &["踢球"],
    date_created = local_date(2022, 11, 29),
    date_modified = local_date(2023, 2, 14),
);
