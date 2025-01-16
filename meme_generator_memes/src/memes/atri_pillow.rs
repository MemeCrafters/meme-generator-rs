use rand::seq::SliceRandom;
use skia_safe::{Color4f, IRect};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::{register_meme, tags::MemeTags};

#[derive(MemeOptions)]
struct Mode {
    /// 模式
    #[option(long, choices=["yes", "no"])]
    mode: Option<String>,

    /// yes 模式
    #[option(short, long, default = false)]
    yes: Option<bool>,

    /// no 模式
    #[option(short, long, default = false)]
    no: Option<bool>,
}

fn atri_pillow(_: Vec<InputImage>, texts: Vec<String>, options: Mode) -> Result<Vec<u8>, Error> {
    let mode = if options.yes.unwrap() {
        "yes"
    } else if options.no.unwrap() {
        "no"
    } else {
        options.mode.as_deref().unwrap_or({
            let mut rng = rand::thread_rng();
            ["yes", "no"].choose(&mut rng).unwrap()
        })
    };
    let text = &texts[0];

    let text_color = match mode {
        "yes" => Color4f::new(1.0, 0.0, 0.0, 0.3),
        _ => Color4f::new(0.0, 0.3, 1.0, 0.3),
    };
    let frame = load_image(format!("atri_pillow/{mode}.png"))?;

    let mut surface = new_surface((300, 150));
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(20, 20, 280, 130),
        text,
        30.0,
        120.0,
        text_params!(
            font_families = &["FZShaoEr-M11S"],
            paint = new_paint(text_color)
        ),
    )?;
    let text_image = surface.image_snapshot();
    let text_image = text_image.rotate(4.0);

    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_image(&text_image, (302, 288), None);
    let border = load_image("atri_pillow/border.png")?;
    canvas.draw_image(&border, (0, 416), None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "atri_pillow",
    atri_pillow,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["ATRI"],
    keywords = &["亚托莉枕头"],
    tags = MemeTags::atri(),
    date_created = local_date(2024, 8, 12),
    date_modified = local_date(2024, 8, 15),
);
