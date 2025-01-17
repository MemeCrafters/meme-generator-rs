use skia_safe::IRect;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn image_subtitle(
    text: &str,
    template_name: &str,
    min_font_size: f32,
    max_font_size: f32,
    text_area_height: i32,
) -> Result<Vec<u8>, Error> {
    let frame = load_image(format!("image_subtitle/{template_name}"))?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    let padding_x = 10;
    let padding_y = 5;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(
            padding_x,
            frame.height() - text_area_height,
            frame.width() - padding_x,
            frame.height() - padding_y,
        ),
        text,
        min_font_size,
        max_font_size,
        None,
    )?;
    encode_png(surface.image_snapshot())
}

macro_rules! register_image_subtitle {
    ($key:expr, $keywords:expr, $default_texts:expr, $template_name:expr, $min_font_size:expr, $max_font_size:expr, $text_area_height:expr, $date_created:expr, $date_modified:expr, $(,)?
    ) => {
        register_meme!(
            $key,
            |_: Vec<InputImage>, texts: Vec<String>, _: NoOptions| -> Result<Vec<u8>, Error> {
                image_subtitle(
                    &texts[0],
                    $template_name,
                    $min_font_size,
                    $max_font_size,
                    $text_area_height,
                )
            },
            min_texts = 1,
            max_texts = 1,
            default_texts = $default_texts,
            keywords = $keywords,
            date_created = $date_created,
            date_modified = $date_modified,
        );
    };
}

register_image_subtitle!(
    "murmur",
    &["低语"],
    &["你的假期余额不足"],
    "murmur.jpg",
    15.0,
    40.0,
    55,
    local_date(2021, 12, 31),
    local_date(2023, 2, 14),
);

register_image_subtitle!(
    "shutup",
    &["别说了"],
    &["你不要再说了"],
    "shutup.jpg",
    15.0,
    40.0,
    80,
    local_date(2022, 1, 19),
    local_date(2023, 2, 14),
);

register_image_subtitle!(
    "slap",
    &["一巴掌"],
    &["你不要再说了"],
    "slap.jpg",
    50.0,
    110.0,
    200,
    local_date(2022, 1, 19),
    local_date(2023, 2, 14),
);

register_image_subtitle!(
    "imprison",
    &["坐牢"],
    &["我发涩图被抓起来了"],
    "imprison.jpg",
    15.0,
    35.0,
    50,
    local_date(2022, 6, 12),
    local_date(2023, 2, 14),
);

register_image_subtitle!(
    "praise",
    &["表扬"],
    &["好，不愧是顶尖运营"],
    "praise.png",
    20.0,
    40.0,
    65,
    local_date(2024, 8, 17),
    local_date(2024, 8, 17),
);

register_image_subtitle!(
    "emperor_dragon",
    &["皇帝龙图"],
    &["听不懂，退朝"],
    "emperor_dragon.png",
    20.0,
    60.0,
    70,
    local_date(2024, 10, 30),
    local_date(2024, 10, 30),
);
