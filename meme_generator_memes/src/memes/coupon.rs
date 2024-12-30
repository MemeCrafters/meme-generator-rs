use skia_safe::{IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn coupon(images: Vec<DecodedImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        let name = &images[0].name;
        &format!("{}陪睡券\n（永久有效）", name)
    };

    let mut surface = new_surface((250, 100));
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(0, 0, 250, 100),
        text,
        15.0,
        30.0,
        None,
    )?;
    let text_image = surface.image_snapshot();

    let func = |images: &Vec<Image>| {
        let frame = load_image("coupon/0.png")?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image = images[0].circle().resize_fit((60, 60), Fit::Cover);
        let image = image.rotate(-22.0);
        let text_image = text_image.rotate(-22.0);
        canvas.draw_image(&image, (164, 85), None);
        canvas.draw_image(&text_image, (94, 108), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "coupon",
    coupon,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    keywords = &["兑换券"],
    date_created = local_date(2022, 3, 12),
    date_modified = local_date(2023, 2, 14),
);
