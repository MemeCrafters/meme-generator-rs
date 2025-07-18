use skia_safe::IRect;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::GifEncoder,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn capoo_say(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let mut encoder = GifEncoder::new();
    for text in texts {
        let mut surface = new_surface((80, 80));
        let canvas = surface.canvas();
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(0, 0, 80, 80),
            text,
            20.0,
            80.0,
            text_params!(font_families = &["FZKaTong-M19S"]),
        )?;
        let text_image = surface.image_snapshot();

        let params = [
            (45, 45, 74, 112, -25),
            (73, 73, 41, 42, -17),
            (80, 80, 43, 36, 0),
            (80, 80, 43, 30, 0),
            (78, 78, 44, 30, 0),
            (78, 78, 44, 29, 0),
        ];

        for i in 0..10 {
            let frame = load_image(format!("capoo_say/{i}.png"))?;
            let mut surface = frame.to_surface();
            let canvas = surface.canvas();
            if (3..9).contains(&i) {
                let (w, h, x, y, angle) = params[i - 3];
                let text_image = text_image.resize_exact((w, h)).rotate(angle as f32);
                canvas.draw_image(&text_image, (x, y), None);
            }
            encoder.add_frame(surface.image_snapshot(), 0.1)?;
        }
    }
    Ok(encoder.finish()?)
}

register_meme! {
    "capoo_say",
    capoo_say,
    min_texts = 1,
    max_texts = 10,
    default_texts = &["寄"],
    keywords = &["咖波说"],
    tags = MemeTags::capoo(),
    date_created = local_date(2023, 3, 28),
    date_modified = local_date(2023, 3, 30),
}
