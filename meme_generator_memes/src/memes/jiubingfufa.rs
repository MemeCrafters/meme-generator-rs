use skia_safe::{Color, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::{GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint, new_stroke_paint},
};

use crate::{options::NoOptions, register_meme};

const DEFAULT_TEXT: &str = "此乃旧病复发也";

fn jiubingfufa(
    images: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };

    let func = |i: usize, images: Vec<Image>| {
        let frame = load_image(format!("jiubingfufa/{i:02}.jpg"))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let img = images[0].circle().resize_exact((120, 120));
        canvas.draw_image(&img, (32, frame.height() - 162), None);
        if i > 9 {
            canvas.draw_text_area_auto_font_size(
                IRect::from_ltrb(0, 0, 290, 160),
                text,
                20.0,
                32.0,
                text_params!(
                    paint = new_paint(Color::WHITE),
                    stroke_paint = new_stroke_paint(Color::BLACK, 3.0),
                ),
            )?;
        }
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 26,
            duration: 0.06,
        },
        None,
    )
}

register_meme!(
    "jiubingfufa",
    jiubingfufa,
    min_images = 1,
    max_images = 1,
    max_texts = 1,
    min_texts = 0,
    keywords = &["旧病复发"],
    default_texts = &[DEFAULT_TEXT],
    date_created = local_date(2025, 4, 1),
    date_modified = local_date(2025, 4, 11),
);
