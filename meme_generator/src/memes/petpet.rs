use skia_safe::Image;

use crate::{
    error::Error,
    manager::register_meme,
    meme::DecodedImage,
    utils::{
        encoder::{make_gif_or_combined_gif, GifInfo},
        image::ImageExt,
        load_image, local_date, new_surface,
        options::Circle,
    },
};

fn petpet(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    options: &Circle,
) -> Result<Vec<u8>, Error> {
    let locs = [
        (14, 20, 98, 98),
        (12, 33, 101, 85),
        (8, 40, 110, 76),
        (10, 33, 102, 84),
        (12, 20, 98, 98),
    ];

    let func = |i: usize, images: &Vec<Image>| {
        let mut image = images[0].square();
        if options.circle {
            image = image.circle();
        }

        let hand = load_image(format!("petpet/{i}.png"))?;
        let mut surface = new_surface(hand.dimensions());
        let canvas = surface.canvas();
        let (x, y, w, h) = locs[i];
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
        None,
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
