use skia_safe::{Color, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

const DEFAULT_TEXT: &str = "采访大佬经验";

fn interview(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };

    let mut surface = new_surface((600, 310));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(20, 200, 580, 310),
        text,
        20.0,
        50.0,
        text_params!(paint = new_paint(Color::BLACK)),
    )?;
    let frame = surface.image_snapshot();
    let huaji = load_image("interview/huaji.png")?;
    let microphone = load_image("interview/microphone.png")?;

    let func = |images: Vec<Image>| {
        let (self_img, user_img) = if images.len() == 2 {
            (&images[0], &images[1])
        } else {
            (&huaji, &images[0])
        };
        let self_img = self_img.square().resize_exact((124, 124));
        let user_img = user_img.square().resize_exact((124, 124));

        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        canvas.draw_image(&microphone, (330, 103), None);
        canvas.draw_image(&self_img, (419, 40), None);
        canvas.draw_image(&user_img, (57, 40), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "interview",
    interview,
    min_images = 1,
    max_images = 2,
    min_texts = 0,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["采访"],
    date_created = local_date(2022, 3, 15),
    date_modified = local_date(2023, 2, 14),
);
