use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_gif_or_combined_gif,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn worship(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("worship/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let img = images[0].square().resize_exact((150, 150));
        let img = img.perspective(&[(0, -30), (135, 17), (135, 145), (0, 140)]);
        canvas.draw_image(&img, (0, 0), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        meme_generator_utils::encoder::GifInfo {
            frame_num: 10,
            duration: 0.04,
        },
        meme_generator_utils::encoder::FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "worship",
    worship,
    min_images = 1,
    max_images = 1,
    keywords = &["膜", "膜拜"],
    date_created = local_date(2022, 2, 10),
    date_modified = local_date(2023, 2, 14),
);
