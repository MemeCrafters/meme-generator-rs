use skia_safe::Image;

use crate::{
    error::Error,
    manager::register_meme,
    meme::DecodedImage,
    utils::{
        encoder::{make_gif_or_combined_gif, GifInfo},
        image::ImageExt,
        load_image, local_date, new_surface,
        options::NoOptions,
    },
};

fn bite(images: &mut Vec<DecodedImage>, _: &Vec<String>, _: &NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (90, 90, 105, 150),
        (90, 83, 96, 172),
        (90, 90, 106, 148),
        (88, 88, 97, 167),
        (90, 85, 89, 179),
        (90, 90, 106, 151),
    ];

    let func = |i: usize, images: &Vec<Image>| {
        let frame = load_image(&format!("bite/{i:02}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        if (0..6).contains(&i) {
            let (w, h, x, y) = locs[i];
            canvas.draw_image(images[0].resize_exact((w, h)), (x, y), None);
        }
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 16,
            duration: 0.07,
        },
        None,
    )
}

register_meme!(
    "bite",
    bite,
    min_images = 1,
    max_images = 1,
    keywords = &["啃"],
    date_created = local_date(2022, 2, 15),
    date_modified = local_date(2023, 2, 14),
);
