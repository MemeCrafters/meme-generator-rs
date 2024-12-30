use chrono::{Local, NaiveDate};
use skia_safe::{textlayout::TextAlign, Color};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{DecodedImage, MemeOptions},
    encoder::encode_png,
    text::Text2Image,
    text_params,
    tools::{load_image, local_date, new_surface},
};

use crate::register_meme;

#[derive(MemeOptions)]
pub(crate) struct Time {
    /// 时间
    #[option(short, long)]
    pub time: Option<String>,
}

fn hold_grudge(_: Vec<DecodedImage>, texts: Vec<String>, options: Time) -> Result<Vec<u8>, Error> {
    let mut time = Local::now().naive_local().date();
    if let Some(time_set) = &options.time {
        if let Ok(t) = NaiveDate::parse_from_str(time_set, "%Y-%m-%d") {
            time = t;
        } else {
            return Err(Error::MemeFeedback(format!(
                "时间格式错误，正确格式为：{}",
                time.format("%Y-%m-%d")
            )));
        }
    }

    let date = time.format("%Y年%m月%d日").to_string();
    let text = format!("{} 晴\n{}\n这个仇我先记下了", date, &texts[0]);
    let mut text2image =
        Text2Image::from_text(&text, 45.0, text_params!(text_align = TextAlign::Left));
    text2image.layout(440.0);
    if text2image.height() > 500.0 {
        return Err(Error::TextOverLength(texts[0].clone()));
    }

    let frame = load_image("hold_grudge/0.png")?;
    let mut surface = new_surface((
        frame.width(),
        frame.height() + text2image.height() as i32 + 20,
    ));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    canvas.draw_image(&frame, (0, 0), None);
    text2image.draw_on_canvas(canvas, (30, frame.height() + 5));

    encode_png(surface.image_snapshot())
}

register_meme!(
    "hold_grudge",
    hold_grudge,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["群友不发涩图"],
    keywords = &["记仇"],
    date_created = local_date(2021, 12, 15),
    date_modified = local_date(2023, 2, 14),
);
