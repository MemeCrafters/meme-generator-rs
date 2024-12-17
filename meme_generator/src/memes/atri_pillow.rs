use rand::seq::SliceRandom;
use skia_safe::{Color4f, IRect};

use crate::{
    error::Error,
    manager::register_meme,
    meme::{DecodedImage, MemeOptions},
    tags::MemeTags,
    utils::{
        canvas::CanvasExt, encoder::encode_png, image::ImageExt, load_image, local_date, new_paint,
        new_surface, text::text_params,
    },
};

#[derive(MemeOptions)]
struct Mode {
    /// 模式
    #[option(long, default="random", choices=["yes", "no", "random"])]
    mode: String,

    /// yes 模式
    #[option(short, long)]
    yes: bool,

    /// no 模式
    #[option(short, long)]
    no: bool,
}

fn atri_pillow(
    _: &mut Vec<DecodedImage>,
    texts: &Vec<String>,
    options: &Mode,
) -> Result<Vec<u8>, Error> {
    let mut mode = if options.yes {
        "yes"
    } else if options.no {
        "no"
    } else {
        options.mode.as_str()
    };
    if mode == "random" {
        let mut rng = rand::thread_rng();
        mode = ["yes", "no"].choose(&mut rng).unwrap();
    }
    let text = texts[0].clone();

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
    encode_png(&surface.image_snapshot())
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
