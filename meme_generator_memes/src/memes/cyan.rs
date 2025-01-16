use skia_safe::{textlayout::TextAlign, Color, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    text_params,
    tools::{local_date, new_paint, new_stroke_paint},
};

use crate::{options::NoOptions, register_meme};

fn cyan(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let color = Color::from_rgb(78, 114, 184);

    let func = |images: Vec<Image>| {
        let img = images[0].square().resize_exact((500, 500));
        let img = img.colorize(color);

        let mut surface = img.to_surface();
        let canvas = surface.canvas();
        canvas
            .draw_text_area(
                IRect::from_ltrb(400, 40, 480, 280),
                "群\n青",
                80.0,
                text_params!(
                    paint = new_paint(Color::WHITE),
                    stroke_paint = new_stroke_paint(color, 6.0)
                ),
            )
            .unwrap();
        canvas
            .draw_text_area(
                IRect::from_ltrb(200, 270, 480, 350),
                "YOASOBI",
                40.0,
                text_params!(
                    paint = new_paint(Color::WHITE),
                    text_align = TextAlign::Right,
                    stroke_paint = new_stroke_paint(color, 5.0)
                ),
            )
            .unwrap();
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "cyan",
    cyan,
    min_images = 1,
    max_images = 1,
    keywords = &["群青"],
    date_created = local_date(2022, 3, 18),
    date_modified = local_date(2023, 2, 14),
);
