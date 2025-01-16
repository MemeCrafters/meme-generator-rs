use chrono::{Datelike, Local, NaiveDate};
use skia_safe::{textlayout::TextAlign, Color, FontStyle, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    text_params,
    tools::{local_date, new_paint, new_surface},
};

use crate::register_meme;

#[derive(MemeOptions)]
struct Time {
    /// 时间
    #[option(short, long)]
    time: Option<String>,
}

const DEFAULT_TEXT: &str = "想玩原神";

fn note_for_leave(
    images: Vec<InputImage>,
    texts: Vec<String>,
    options: Time,
) -> Result<Vec<u8>, Error> {
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
    let name = &images[0].name;
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };

    let mut surface = new_surface((800, 950));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    canvas
        .draw_text_area(
            IRect::from_ltrb(40, 20, 760, 180),
            "请假条",
            100.0,
            text_params!(font_style = FontStyle::bold()),
        )
        .unwrap();
    canvas
        .draw_bbcode_text_area(
            IRect::from_ltrb(40, 180, 760, 270),
            format!("本人[u] {name} [/u]因"),
            50.0,
            text_params!(text_align = TextAlign::Left),
        )
        .map_err(|_| Error::TextOverLength(name.to_string()))?;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(40, 300, 285, 700),
        text,
        40.0,
        90.0,
        text_params!(paint = new_paint(Color::RED)),
    )?;
    let leave_text = format!(
        "于[u] {} [/u]年[u] {} [/u]月[u] {} [/u]日请假一天",
        time.year(),
        time.month(),
        time.day()
    );
    canvas.draw_bbcode_text((40, 720), &leave_text, 50.0, None);
    canvas.draw_text(
        (40, 800),
        "望领导批准！！！",
        75.0,
        text_params!(font_style = FontStyle::bold()),
    );
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image = images[0].resize_fit((450, 400), Fit::Contain);
        canvas.draw_image(&image, (300, 290), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "note_for_leave",
    note_for_leave,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["请假条"],
    date_created = local_date(2023, 4, 27),
    date_modified = local_date(2023, 4, 27),
);
