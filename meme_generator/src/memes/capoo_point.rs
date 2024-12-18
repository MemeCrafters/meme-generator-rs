use skia_safe::{Color, Image};

use crate::{
    error::Error,
    manager::register_meme,
    meme::DecodedImage,
    tags::MemeTags,
    utils::{
        encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
        image::ImageExt,
        load_image, local_date, new_surface,
        options::NoOptions,
    },
};

fn capoo_point(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let locs = [
        (165, 167, 57, 290),
        (165, 167, 53, 290),
        (160, 165, 57, 293),
        (165, 167, 56, 290),
    ];

    let func = |i: usize, images: &Vec<Image>| {
        let frame = load_image(format!("capoo_point/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let (w, h, x, y) = locs[i];
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
    "capoo_point",
    capoo_point,
    min_images = 1,
    max_images = 1,
    keywords = &["咖波指"],
    tags = MemeTags::capoo(),
    date_created = local_date(2024, 10, 24),
    date_modified = local_date(2024, 10, 24),
}
