use chrono::{Datelike, Local, NaiveDate};
use skia_safe::IRect;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    canvas::CanvasExt,
    encoder::encode_png,
    image::{Fit, ImageExt},
    tools::{load_image, local_date},
};

use crate::register_meme;

#[derive(MemeOptions)]
struct Time {
    /// 时间
    #[option(short, long)]
    time: Option<String>,
}

fn abstinence(images: Vec<InputImage>, _: Vec<String>, options: Time) -> Result<Vec<u8>, Error> {
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

    let frame = load_image("abstinence/base.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();

    canvas
        .draw_bbcode_text_area_auto_font_size(
            IRect::from_ltrb(150, 650, 760, 800),
            format!("戒导人：[u]{name}[/u]"),
            10.0,
            20.0,
            None,
        )
        .map_err(|_| Error::TextOverLength(name.to_string()))?;
    canvas
        .draw_bbcode_text_area(
            IRect::from_ltrb(150, 750, 760, 800),
            format!(
                "[u] {} [/u]年[u] {} [/u]月[u] {} [/u]日",
                time.year(),
                time.month(),
                time.day()
            ),
            20.0,
            None,
        )
        .unwrap();
    let stamp = load_image("abstinence/stamp.png")?;

    let image = images[0].image.resize_fit((270, 360), Fit::Contain);
    canvas.draw_image(&image, (80, 380), None);
    canvas.draw_image(&stamp, (310, 660), None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "abstinence",
    abstinence,
    min_images = 1,
    max_images = 1,
    keywords = &["戒导"],
    date_created = local_date(2024, 12, 13),
    date_modified = local_date(2024, 12, 14),
);
