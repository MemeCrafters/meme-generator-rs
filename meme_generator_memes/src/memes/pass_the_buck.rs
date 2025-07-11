use skia_safe::IRect;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn pass_the_buck(
    images: Vec<InputImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let locs = [
        (2, 26),
        (10, 24),
        (15, 27),
        (17, 29),
        (10, 20),
        (2, 29),
        (3, 31),
        (1, 30),
    ];
    let img = images[0].image.square().resize_exact((27, 27));

    let mut encoder = GifEncoder::new();
    for i in 0..8 {
        let frame = load_image(format!("pass_the_buck/{i}.png"))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        if !texts.is_empty() {
            let text = &texts[0];
            canvas.draw_text_area_auto_font_size(
                IRect::from_ltrb(0, 0, frame.width(), 20),
                text,
                10.0,
                20.0,
                None,
            )?;
        }
        canvas.draw_image(&img, locs[i], None);
        encoder.add_frame(surface.image_snapshot(), 0.1)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "pass_the_buck",
    pass_the_buck,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &["你写!"],
    keywords = &["推锅", "甩锅"],
    date_created = local_date(2023, 3, 31),
    date_modified = local_date(2023, 4, 18),
);
