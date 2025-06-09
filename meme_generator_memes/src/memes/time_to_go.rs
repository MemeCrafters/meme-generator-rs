use skia_safe::{Color, IRect};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

const DEFAULT_TEXT: &str = "说完了吗？该走了";

fn time_to_go(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };

    let frame = load_image("time_to_go/0.png")?;

    let mut surface = new_surface(frame.dimensions());
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    let img = images[0].image.square().resize_exact((105, 105));
    canvas.draw_image(&img, (230, 82), None);
    canvas.draw_image(&frame, (0, 0), None);
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(20, 232, 330, 312),
        text,
        20.0,
        40.0,
        None,
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "time_to_go",
    time_to_go,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["该走了"],
    date_created = local_date(2024, 9, 4),
    date_modified = local_date(2024, 9, 4),
);
