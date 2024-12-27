use rand::Rng;
use skia_safe::IRect;

use crate::{
    error::Error,
    manager::register_meme,
    meme::{DecodedImage, MemeOptions},
    utils::{
        canvas::CanvasExt, encoder::encode_png, image::ImageExt, load_image, local_date,
        tags::MemeTags,
    },
};

#[derive(MemeOptions)]
struct Number {
    /// 图片编号
    #[option(short, long, minimum = 0, maximum = 13)]
    number: i32,
}

fn jinhsi(
    _: &mut Vec<DecodedImage>,
    texts: &Vec<String>,
    options: &Number,
) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let mut num = options.number;
    if num == 0 {
        let mut rng = rand::thread_rng();
        num = rng.gen_range(1..=13);
    }

    let frame = load_image(format!("jinhsi/{:02}.png", num))?;
    let paddings = [55, 43, 50, 36, 40, 33, 36, 38, 33, 46, 26, 33, 28];
    let padding = paddings[(num - 1) as usize];

    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(0, frame.height() - padding, frame.width(), frame.height()),
        text,
        15.0,
        50.0,
        None,
    )?;

    encode_png(&surface.image_snapshot())
}

register_meme!(
    "jinhsi",
    jinhsi,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["汐汐"],
    keywords = &["汐汐", "今汐"],
    tags = MemeTags::jinhsi(),
    date_created = local_date(2024, 12, 7),
    date_modified = local_date(2024, 12, 7),
);
