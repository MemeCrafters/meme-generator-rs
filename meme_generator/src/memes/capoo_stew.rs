use skia_safe::Image;

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

fn capoo_stew(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: &Vec<Image>| {
        let frame = load_image(format!("capoo_stew/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        let image = images[0].circle().resize_exact((80, 80));
        let y = if [2, 3, 5].contains(&i) { 45 } else { 47 };
        canvas.draw_image(&image, (88, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 5,
            duration: 0.08,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "capoo_stew",
    capoo_stew,
    max_images = 1,
    min_images = 1,
    tags = MemeTags::capoo(),
    keywords = &["咖波炖"],
    date_created = local_date(2024, 8, 23),
    date_modified = local_date(2024, 8, 23),
}