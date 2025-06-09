use skia_safe::IRect;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

const DEFAULT_TEXT: &str = "怎么说话的你";

fn beat_head(images: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };
    let image = images[0].image.circle();

    let locs = [(160, 121, 76, 76), (172, 124, 69, 69), (208, 166, 52, 52)];

    let mut encoder = GifEncoder::new();
    for i in 0..3 {
        let (x, y, w, h) = locs[i];
        let head = image.resize_exact((w, h));
        let frame = load_image(format!("beat_head/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&head, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(175, 28, 316, 82),
            text,
            10.0,
            50.0,
            None,
        )?;
        encoder.add_frame(surface.image_snapshot(), 0.05)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "beat_head",
    beat_head,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["拍头"],
    date_created = local_date(2023, 3, 8),
    date_modified = local_date(2023, 3, 8),
);
