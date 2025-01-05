use skia_safe::{Color, IRect};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme};

fn nijika_holdsign(_: Vec<NamedImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let frame = load_image("nijika_holdsign/0.jpg")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(57, 279, 249, 405),
        text,
        25.0,
        60.0,
        text_params!(
            font_families = &["FZSJ-QINGCRJ"],
            paint = new_paint(Color::from_rgb(111, 95, 95))
        ),
    )?;
    encode_png(surface.image_snapshot())
}

register_meme!(
    "nijika_holdsign",
    nijika_holdsign,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["你可少看点二次元吧"],
    keywords = &["伊地知虹夏举牌", "虹夏举牌"],
    date_created = local_date(2023, 6, 20),
    date_modified = local_date(2023, 6, 20),
);
