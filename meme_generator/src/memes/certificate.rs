use chrono::{Local, NaiveDate};
use skia_safe::{textlayout::TextAlign, IRect};

use crate::{
    error::Error,
    manager::register_meme,
    meme::DecodedImage,
    utils::{
        canvas::CanvasExt, color_from_hex_code, encoder::encode_png, image::ImageExt, load_image,
        local_date, new_paint, options::Time, text::text_params,
    },
};

fn certificate(
    _: &mut Vec<DecodedImage>,
    texts: &Vec<String>,
    options: &Time,
) -> Result<Vec<u8>, Error> {
    let mut time = Local::now().naive_local().date();
    if !options.time.is_empty() {
        if let Ok(t) = NaiveDate::parse_from_str(&options.time, "%Y-%m-%d") {
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

    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(1565, 1520, 1700, 1595),
        &time.format("%Y").to_string(),
        40.0,
        60.0,
        None,
    )?;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(1752, 1520, 1816, 1595),
        &time.format("%m").to_string(),
        40.0,
        60.0,
        None,
    )?;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(1865, 1520, 1930, 1595),
        &time.format("%d").to_string(),
        40.0,
        60.0,
        None,
    )?;

    encode_png(&surface.image_snapshot())
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
