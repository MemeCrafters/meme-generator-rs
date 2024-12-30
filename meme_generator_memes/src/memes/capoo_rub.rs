use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn capoo_rub(images: Vec<DecodedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (178, 184, 78, 260),
        (178, 174, 84, 269),
        (178, 174, 84, 269),
        (178, 178, 84, 264),
    ];

    let func = |i: usize, images: &Vec<Image>| {
        let (w, h, x, y) = locs[i];
        let frame = load_image(format!("capoo_rub/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].square().resize_exact((w, h));
        canvas.draw_image(&image, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 4,
            duration: 0.1,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "capoo_rub",
    capoo_rub,
    min_images = 1,
    max_images = 1,
    keywords = &["咖波蹭", "咖波贴"],
    tags = MemeTags::capoo(),
    date_created = local_date(2022, 11, 29),
    date_modified = local_date(2023, 2, 14),
}
