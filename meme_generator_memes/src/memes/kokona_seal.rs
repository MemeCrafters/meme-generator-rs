use rand::RngExt;
use skia_safe::IRect;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{color_from_hex_code, load_image, local_date, new_paint, new_surface},
};

use crate::{options::number_option, register_meme, tags::MemeTags};

number_option!(Number, 1, 12);

fn kokona_seal(_: Vec<InputImage>, texts: Vec<String>, options: Number) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let num = options.number.unwrap_or({
        rand::rng().random_range(1..=12)
    });

    let size = (320, 155);
    let loc = (75, 25);
    let mut surface = new_surface(size);
    let canvas = surface.canvas();
    let padding = 10;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(padding, padding, size.0 - padding, size.1 - padding),
        text,
        50.0,
        150.0,
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            paint = new_paint(color_from_hex_code("#fe0000")),
        ),
    )?;
    let text_image = surface.image_snapshot().rotate(-16.0);

    let frame = load_image(format!("kokona_seal/{num:02}.png"))?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_image(&text_image, loc, None);

    encode_png(surface.image_snapshot())
}

register_meme!(
    "kokona_seal",
    kokona_seal,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["满分"],
    keywords = &["心奈印章"],
    tags = MemeTags::kokona(),
    date_created = local_date(2024, 11, 5),
    date_modified = local_date(2024, 11, 22),
);
