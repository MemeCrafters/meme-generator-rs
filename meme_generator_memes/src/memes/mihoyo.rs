use skia_safe::{Color, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme};

fn mihoyo(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let logo = load_image("mihoyo/0.png")?.resize_height(50);

    let func = |images: Vec<Image>| {
        let img = images[0].square().resize_exact((500, 500));
        let mut surface = img.to_surface();
        let canvas = surface.canvas();
        canvas.draw_irect(
            IRect::from_xywh(0, 440, 500, 60),
            &new_paint(Color::from_argb(230, 53, 49, 65)),
        );
        canvas.draw_image(&logo, ((img.width() - logo.width()) / 2, 445), None);
        Ok(surface.image_snapshot().round_corner(100.0))
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "mihoyo",
    mihoyo,
    min_images = 1,
    max_images = 1,
    keywords = &["米哈游"],
    date_created = local_date(2023, 5, 6),
    date_modified = local_date(2023, 5, 6),
);
