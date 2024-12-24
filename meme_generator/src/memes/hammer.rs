use skia_safe::{Color, Image};

use crate::{
    error::Error,
    manager::register_meme,
    meme::DecodedImage,
    utils::{
        encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
        image::ImageExt,
        load_image, local_date, new_surface,
        options::NoOptions,
    },
};

fn hammer(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let locs = [
        (62, 143, 158, 113),
        (52, 177, 173, 105),
        (42, 192, 192, 92),
        (46, 182, 184, 100),
        (54, 169, 174, 110),
        (69, 128, 144, 135),
        (65, 130, 152, 124),
    ];

    let func = |i: usize, images: &Vec<Image>| {
        let frame = load_image(format!("hammer/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        let (x, y, w, h) = locs[i];
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
            frame_num: 7,
            duration: 0.07,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "hammer",
    hammer,
    min_images = 1,
    max_images = 1,
    keywords = &["锤"],
    date_created = local_date(2022, 4, 20),
    date_modified = local_date(2023, 2, 14),
);
