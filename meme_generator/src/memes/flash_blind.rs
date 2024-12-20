use skia_safe::{IRect, Image};

use crate::{
    error::Error,
    manager::register_meme,
    meme::DecodedImage,
    utils::{
        encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
        image::ImageExt,
        local_date,
        options::NoOptions,
    },
};

fn flash_blind(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: &Vec<Image>| {
        let image = images[0].clone();
        match i {
            0 => Ok(image),
            1 => Ok(image.invert()),
            2 => {
                let width = image.width();
                let height = image.height();
                let padding_w = width / 20;
                let padding_h = height / 20;
                let image = image
                    .crop(IRect::new(
                        padding_w,
                        padding_h,
                        image.width() - padding_w,
                        image.height() - padding_h,
                    ))
                    .resize_exact((width, height));
                Ok(image)
            }
            3 => Ok(image.invert()),
            _ => unreachable!(),
        }
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 4,
            duration: 0.03,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "flash_blind",
    flash_blind,
    min_images = 1,
    max_images = 1,
    keywords = &["闪瞎"],
    date_created = local_date(2023, 5, 5),
    date_modified = local_date(2023, 5, 5),
);
