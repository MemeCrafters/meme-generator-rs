use skia_safe::{Color, IRect};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme};

fn luxun_say(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let frame = load_image("luxun_say/0.jpg")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(
            40,
            frame.height() - 200,
            frame.width() - 40,
            frame.height() - 100,
        ),
        text,
        30.0,
        40.0,
        text_params!(paint = new_paint(Color::WHITE)),
    )?;
    canvas.draw_text(
        (320, 400),
        "--鲁迅",
        30.0,
        text_params!(paint = new_paint(Color::WHITE)),
    );
    encode_png(surface.image_snapshot())
}

register_meme!(
    "luxun_say",
    luxun_say,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["我没有说过这句话"],
    keywords = &["鲁迅说", "鲁迅说过"],
    date_created = local_date(2021, 12, 15),
    date_modified = local_date(2023, 2, 14),
);
