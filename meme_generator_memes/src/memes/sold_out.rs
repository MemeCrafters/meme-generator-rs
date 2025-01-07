use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme};

fn sold_out(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let icon = load_image("sold_out/0.png")?;

    let func = |images: Vec<Image>| {
        let img = &images[0];
        let img = if img.width() > img.height() {
            img.resize_height(600)
        } else {
            img.resize_width(600)
        };

        let mut surface = img.to_surface();
        let canvas = surface.canvas();
        canvas.draw_irect(img.bounds(), &new_paint(Color::from_argb(80, 0, 0, 0)));
        canvas.draw_image(
            &icon,
            (
                (img.width() - icon.height()) / 2,
                (img.height() - icon.height()) / 2,
            ),
            None,
        );

        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "sold_out",
    sold_out,
    min_images = 1,
    max_images = 1,
    keywords = &["卖掉了"],
    date_created = local_date(2024, 11, 18),
    date_modified = local_date(2024, 11, 18),
);
