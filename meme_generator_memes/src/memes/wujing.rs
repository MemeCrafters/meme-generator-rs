use skia_safe::{textlayout::TextAlign, Color, IRect};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    shortcut, text_params,
    tools::{load_image, local_date, new_paint, new_stroke_paint},
};

use crate::{options::NoOptions, register_meme};

fn wujing(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("wujing/0.jpg")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();

    let draw = |pos: (i32, i32, i32, i32), text: &str, align: TextAlign| {
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(pos.0, pos.1, pos.2, pos.3),
            text,
            50.0,
            100.0,
            text_params!(
                text_align = align,
                paint = new_paint(Color::WHITE),
                stroke_paint = new_stroke_paint(Color::BLACK, 10.0),
            ),
        )
    };

    if texts.len() == 2 {
        draw((20, 560, 350, 690), &texts[0], TextAlign::Right)?;
        draw((610, 540, 917, 670), &texts[1], TextAlign::Left)?;
    } else {
        draw((50, 430, 887, 550), &texts[0], TextAlign::Left)?;
        draw((20, 560, 350, 690), &texts[1], TextAlign::Right)?;
        draw((610, 540, 917, 670), &texts[2], TextAlign::Left)?;
        if texts.len() >= 4 {
            draw((50, 680, 887, 810), &texts[3], TextAlign::Center)?;
        }
    }

    encode_png(surface.image_snapshot())
}

register_meme!(
    "wujing",
    wujing,
    min_texts = 2,
    max_texts = 4,
    default_texts = &["不买华为不是", "人"],
    keywords = &["吴京中国"],
    shortcuts = &[shortcut!(
        r"吴京[\s:：]*(?P<left>\S*)中国(?P<right>\S*)",
        texts = &["{left}", "{right}"],
        humanized = "吴京xx中国xx",
    )],
    date_created = local_date(2022, 6, 12),
    date_modified = local_date(2023, 2, 14),
);
