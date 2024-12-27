use rand::seq::SliceRandom;
use skia_safe::{Color, IRect};

use crate::{
    error::Error,
    manager::register_meme,
    meme::{DecodedImage, MemeOptions},
    utils::{
        canvas::CanvasExt, encoder::encode_png, image::ImageExt, load_image, local_date, new_paint,
        tags::MemeTags, text::text_params,
    },
};

#[derive(MemeOptions)]
struct Position {
    /// 消息框的位置
    #[option(short, long, default="random", choices=["left", "right", "random"])]
    position: String,

    /// 左
    #[option(short, long)]
    left: bool,

    /// 右
    #[option(short, long)]
    right: bool,
}

fn kokona_say(
    _: &mut Vec<DecodedImage>,
    texts: &Vec<String>,
    options: &Position,
) -> Result<Vec<u8>, Error> {
    let mut position = if options.left {
        "left"
    } else if options.right {
        "right"
    } else {
        options.position.as_str()
    };
    if position == "random" {
        let mut rng = rand::thread_rng();
        position = ["left", "right"].choose(&mut rng).unwrap();
    }
    let text = &texts[0];

    let img_name = match position {
        "left" => "01.png",
        _ => "02.png",
    };
    let frame = load_image(format!("kokona_say/{}", img_name))?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    let (left, right) = if position == "left" {
        (50, 650)
    } else {
        (frame.width() - 650, frame.width() - 50)
    };
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(left, 0, right, 220),
        text,
        50.0,
        100.0,
        text_params!(paint = new_paint(Color::BLACK)),
    )?;

    encode_png(&surface.image_snapshot())
}

register_meme!(
    "kokona_say",
    kokona_say,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["那我问你"],
    keywords = &["心奈说"],
    tags = MemeTags::kokona(),
    date_created = local_date(2024, 12, 12),
    date_modified = local_date(2024, 12, 12),
);
