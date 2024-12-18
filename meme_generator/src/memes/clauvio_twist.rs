use skia_safe::Image;

use crate::{
    error::Error,
    manager::register_meme,
    meme::DecodedImage,
    utils::{
        encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
        image::ImageExt,
        load_image, local_date,
        options::NoOptions,
    },
};

fn clauvio_twist(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let params = [
        (0, (45, 144)),
        (0, (45, 144)),
        (0, (45, 144)),
        (2, (45, 141)),
        (4, (45, 141)),
        (8, (40, 141)),
        (12, (38, 142)),
        (30, (32, 148)),
        (75, (25, 158)),
        (115, (0, 160)),
        (130, (0, 160)),
        (125, (0, 155)),
        (120, (0, 150)),
        (115, (0, 148)),
        (110, (5, 146)),
        (85, (14, 146)),
        (70, (19, 146)),
        (45, (28, 144)),
        (37, (38, 141)),
        (10, (42, 144)),
    ];

    let func = |i: usize, images: &Vec<Image>| {
        let frame = load_image(&format!("clauvio_twist/{i:02}.png"))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let (angle, pos) = params[i % 20];
        let image = images[0]
            .circle()
            .resize_exact((100, 100))
            .rotate(-angle as f32);
        canvas.draw_image(&image, pos, None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 40,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "clauvio_twist",
    clauvio_twist,
    min_images = 1,
    max_images = 1,
    keywords = &["鼠鼠搓"],
    date_created = local_date(2024, 8, 31),
    date_modified = local_date(2024, 8, 31),
);
