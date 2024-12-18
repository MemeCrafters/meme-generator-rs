use skia_safe::{Color, Image};

use crate::{
    error::Error,
    manager::register_meme,
    meme::DecodedImage,
    tags::MemeTags,
    utils::{
        encoder::make_png_or_gif,
        image::{Fit, ImageExt},
        load_image, local_date, new_surface,
        options::NoOptions,
    },
};

fn captain(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let bg0 = load_image("captain/0.png")?;
    let bg1 = load_image("captain/1.png")?;
    let bg2 = load_image("captain/2.png")?;

    let func = |images: &Vec<Image>| {
        let image_num = images.len();
        let iter_num = if image_num == 2 { 3 } else { image_num };
        let mut surface = new_surface((640, 440 * iter_num as i32));
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        for i in 0..iter_num {
            let bg = if i == iter_num - 1 {
                &bg2
            } else if i == iter_num - 2 {
                &bg1
            } else {
                &bg0
            };
            let image = if i >= image_num {
                &images[image_num - 1]
            } else {
                &images[i]
            };
            let image = image.resize_fit((250, 350), Fit::Contain);
            canvas.draw_image(bg, (0, 440 * i as i32), None);
            canvas.draw_image(&image, (350, 35 + 440 * i as i32), None);
        }
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme! {
    "captain",
    captain,
    min_images = 2,
    max_images = 5,
    tags = MemeTags::captain(),
    keywords = &["舰长"],
    date_created = local_date(2022, 10, 9),
    date_modified = local_date(2023, 2, 14),
}
