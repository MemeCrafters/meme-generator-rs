use chrono::{Local, NaiveDate};
use skia_safe::{IRect, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{color_from_hex_code, load_image, local_date, new_paint},
};

use crate::register_meme;

#[derive(MemeOptions)]
pub(crate) struct Time {
    /// 时间
    #[option(short, long)]
    pub time: Option<String>,
}

fn certificate(_: Vec<InputImage>, texts: Vec<String>, options: Time) -> Result<Vec<u8>, Error> {
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

    let frame = load_image("certificate/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(340, 660, 840, 800),
        &texts[0],
        20.0,
        80.0,
        None,
    )?;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(565, 1040, 2100, 1320),
        &texts[1],
        60.0,
        120.0,
        text_params!(paint = new_paint(color_from_hex_code("#ff0000"))),
    )?;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(1500, 1400, 2020, 1520),
        &texts[2],
        20.0,
        60.0,
        None,
    )?;

    let text = if texts.len() >= 4 {
        &texts[3]
    } else {
        "　　在本学年第一学期中表现优秀，被我校决定评为"
    };
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(450, 850, 2270, 1080),
        text,
        40.0,
        80.0,
        text_params!(text_align = TextAlign::Left),
    )?;

    canvas
        .draw_text_area(
            IRect::from_ltrb(1565, 1520, 1700, 1600),
            &time.format("%Y").to_string(),
            50.0,
            None,
        )
        .unwrap();
    canvas
        .draw_text_area(
            IRect::from_ltrb(1752, 1520, 1816, 1600),
            &time.format("%m").to_string(),
            50.0,
            None,
        )
        .unwrap();
    canvas
        .draw_text_area(
            IRect::from_ltrb(1865, 1520, 1930, 1600),
            &time.format("%d").to_string(),
            50.0,
            None,
        )
        .unwrap();

    encode_png(surface.image_snapshot())
}

register_meme!(
    "certificate",
    certificate,
    min_texts = 3,
    max_texts = 4,
    default_texts = &["小王", "优秀学生", "一年一班"],
    keywords = &["奖状", "证书"],
    date_created = local_date(2023, 12, 3),
    date_modified = local_date(2023, 12, 3),
);
