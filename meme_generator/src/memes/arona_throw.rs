use skia_safe::Image;

use crate::{
    error::Error,
    manager::register_meme,
    meme::DecodedImage,
    tags::MemeTags,
    utils::{
        encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
        image::{Fit, ImageExt},
        load_image, local_date,
        options::NoOptions,
    },
};

fn arona_throw(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let position_list = [
        (270, 295),
        (154, 291),
        (154, 291),
        (89, 211),
        (41, 195),
        (28, 192),
        (16, 200),
        (-10, 206),
        (-40, 210),
        (-80, 214),
        (324, 245),
        (324, 256),
        (331, 251),
        (331, 251),
        (318, 260),
        (318, 260),
    ];
    let position_list2 = [(324, 15), (324, 106), (324, 161), (324, 192)];

    let func = |i: usize, images: &Vec<Image>| {
        let pyroxenes = images[0].resize_fit((120, 120), Fit::Cover).circle();
        let arona = load_image(format!("arona_throw/{i:02}.png"))?;
        let mut surface = arona.to_surface();
        let canvas = surface.canvas();
        canvas.draw_image(&pyroxenes, position_list[i], None);
        if (6..=9).contains(&i) {
            canvas.draw_image(&pyroxenes, position_list2[i - 6], None);
        }
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 16,
            duration: 0.04,
        },
        FrameAlign::NoExtend,
    )
}

register_meme!(
    "arona_throw",
    arona_throw,
    min_images = 1,
    max_images = 1,
    keywords = &["阿罗娜扔"],
    tags = MemeTags::arona(),
    date_created = local_date(2024, 12, 10),
    date_modified = local_date(2024, 12, 10),
);
