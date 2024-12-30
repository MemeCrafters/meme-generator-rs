use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::Circle, register_meme};

fn petpet(images: Vec<DecodedImage>, _: Vec<String>, options: Circle) -> Result<Vec<u8>, Error> {
    let locs = [
        (14, 20, 98, 98),
        (12, 33, 101, 85),
        (8, 40, 110, 76),
        (10, 33, 102, 84),
        (12, 20, 98, 98),
    ];

    let func = |i: usize, images: &Vec<Image>| {
        let hand = load_image(format!("petpet/{i}.png"))?;
        let mut surface = new_surface(hand.dimensions());
        let canvas = surface.canvas();
        let (x, y, w, h) = locs[i];
        let image = images[0].square();
        let image = if options.circle.unwrap_or(false) {
            image.circle()
        } else {
            image
        };
        let image = image.resize_exact((w, h));
        canvas.draw_image(&image, (x, y), None);
        canvas.draw_image(&hand, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 5,
            duration: 0.06,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "petpet",
    petpet,
    min_images = 1,
    max_images = 1,
    keywords = &["摸", "摸摸", "摸头", "rua"],
    date_created = local_date(2021, 8, 1),
    date_modified = local_date(2021, 8, 1),
);
