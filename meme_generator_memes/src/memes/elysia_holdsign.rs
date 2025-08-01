use rand::Rng;
use skia_safe::{IRect, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{
        color_from_hex_code, load_image, local_date, new_paint, new_stroke_paint, new_surface,
    },
};

use crate::{options::number_option, register_meme, tags::MemeTags};

const DEFAULT_TEXT: &str = "要好好爱莉哦~";

number_option!(Number, 1, 8);

fn elysia_holdsign(
    _: Vec<InputImage>,
    texts: Vec<String>,
    options: Number,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() { &texts[0] } else { DEFAULT_TEXT };

    let img_num = 8;
    let num = match options.number {
        None => rand::thread_rng().gen_range(1..=img_num),
        Some(n) => {
            if n < 1 || n > img_num {
                return Err(Error::MemeFeedback(format!("图片编号错误，请输入 1-{}", img_num)));
            }
            n
        }
    };

    let text_loc = [
        ((300, 200), (144, 322), [(0, 66), (276, 0), (319, 178), (43, 244)]),
        ((300, 250), (-46, -50), [(0, 83), (312, 0), (348, 243), (46, 314)]),
        ((300, 150), (106, 351), [(0, 0), (286, 0), (276, 149), (12, 149)]),
        ((250, 200), (245, -6), [(31, 0), (288, 49), (256, 239), (0, 190)]),
        ((500, 200), (0, 0), [(0, 0), (492, 0), (462, 198), (25, 198)]),
        ((350, 150), (74, 359), [(0, 52), (345, 0), (364, 143), (31, 193)]),
        ((270, 200), (231, -9), [(31, 0), (305, 49), (270, 245), (0, 192)]),
        ((350, 150), (64, 340), [(0, 44), (345, 0), (358, 153), (34, 197)]),
    ];

    let frame = load_image(format!("elysia_holdsign/{num:02}.png"))?;
    let (size, loc, points) = text_loc[num as usize - 1];

    let mut text_surface = new_surface(size);
    let canvas = text_surface.canvas();
    let padding = 10;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(padding, padding, size.0 - padding, size.1 - padding),
        text,
        30.0,
        80.0,
        text_params!(
            text_align = TextAlign::Center,
            font_families = &["FZShaoEr-M11S"],
            paint = new_paint(color_from_hex_code("#3b0b07")),
            stroke_paint = new_stroke_paint(color_from_hex_code("#ff5995"), 1.0),
        ),
    )?;
    let text_image = text_surface.image_snapshot();

    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_image(&text_image.perspective(&points), loc, None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "elysia_holdsign",
    elysia_holdsign,
    tags = MemeTags::elysia(),
    min_texts = 1,
    max_texts = 1,
    keywords = &["爱莉举牌"],
    default_texts = &[DEFAULT_TEXT],
    date_created = local_date(2024, 6, 1),
    date_modified = local_date(2024, 6, 1),
);
