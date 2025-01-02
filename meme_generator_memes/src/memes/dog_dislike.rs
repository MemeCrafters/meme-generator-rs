use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::{make_gif_or_combined_gif, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::Circle, register_meme};

fn dog_dislike(images: Vec<NamedImage>, _: Vec<String>, options: Circle) -> Result<Vec<u8>, Error> {
    let locs = [
        (36, 408),
        (36, 410),
        (40, 375),
        (40, 355),
        (36, 325),
        (28, 305),
        (28, 305),
        (28, 305),
        (28, 305),
        (28, 285),
        (28, 285),
        (28, 285),
        (28, 285),
        (28, 290),
        (30, 295),
        (30, 300),
        (30, 300),
        (30, 300),
        (30, 300),
        (30, 300),
        (30, 300),
        (28, 298),
        (26, 296),
        (24, 294),
        (28, 294),
        (26, 294),
        (24, 294),
        (35, 294),
        (115, 330),
        (150, 355),
        (180, 420),
        (180, 450),
        (150, 450),
        (150, 450),
    ];

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("dog_dislike/{i:02}.png"))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image = images[0].square().resize_exact((122, 122));
        let image = if options.circle.unwrap() {
            image.circle()
        } else {
            image
        };
        canvas.draw_image(&image, locs[i], None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 34,
            duration: 0.08,
        },
        None,
    )
}

register_meme!(
    "dog_dislike",
    dog_dislike,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 0,
    keywords = &["狗都不玩"],
    date_created = local_date(2023, 11, 16),
    date_modified = local_date(2023, 11, 16),
);
