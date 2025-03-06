use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn telescope(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: Vec<Image>| {
        let img = images[0].square();
        let img_big = img.resize_exact((600, 600));
        let img_small = img.resize_exact((230, 230));
        let frame = load_image(format!("telescope/{i:02}.png"))?;

        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);

        match i {
            4..=17 => {
                let x = -167 + (i as i32 - 4) * 4;
                let y = -361 + (i as i32 - 4) * 7;
                canvas.draw_image(&img_big, (x, y), None);
            }
            23..=37 => {
                let x = -90 + (i as i32 - 23) * 5;
                let y = -245 + (i as i32 - 23) * 5;
                canvas.draw_image(&img_big, (x, y), None);
            }
            43..=45 => {
                canvas.draw_image(&img_big, (0, -210), None);
            }
            46..=56 => {
                canvas.draw_image(&img_small, (8, -21), None);
            }
            _ => {}
        }

        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 69,
            duration: 0.1,
        },
        FrameAlign::ExtendFirst,
    )
}

register_meme!(
    "telescope",
    telescope,
    min_images = 1,
    max_images = 1,
    keywords = &["望远镜"],
    date_created = local_date(2024, 1, 18),
    date_modified = local_date(2024, 1, 18),
);
