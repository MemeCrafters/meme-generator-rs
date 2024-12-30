use skia_safe::{Canvas, Point};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    encoder::encode_png,
    image::ImageExt,
    text::Text2Image,
    text_params,
    tools::{color_from_hex_code, load_image, local_date, new_paint, new_stroke_paint},
};

use crate::{options::NoOptions, register_meme};

struct ShadowText {
    inner: Text2Image,
    shadow: Text2Image,
    shadow_width: f32,
}

impl ShadowText {
    fn new(text: String, fontsize: f32, shadow_width: f32) -> Self {
        let font_families = &["PangMenZhengDao-Cu"];
        let inner = Text2Image::from_text(
            &text,
            fontsize,
            text_params!(
                font_families = font_families,
                paint = new_paint(color_from_hex_code("#e60012")),
                stroke_paint = new_stroke_paint(color_from_hex_code("#500000"), 8.0),
            ),
        );
        let shadow = Text2Image::from_text(
            &text,
            fontsize,
            text_params!(
                font_families = font_families,
                paint = new_paint(color_from_hex_code("#500000")),
                stroke_paint = new_stroke_paint(color_from_hex_code("#500000"), shadow_width),
            ),
        );
        Self {
            inner,
            shadow,
            shadow_width,
        }
    }

    fn width(&self) -> f32 {
        self.inner.longest_line()
    }

    fn height(&self) -> f32 {
        self.inner.height()
    }

    fn draw_on_canvas(&self, canvas: &Canvas, origin: impl Into<Point>) {
        let origin: Point = origin.into();
        self.shadow.draw_on_canvas(
            canvas,
            (
                origin.x + self.shadow_width * 0.25,
                origin.y + self.shadow_width * 1.0,
            ),
        );
        self.inner.draw_on_canvas(canvas, origin);
    }
}

fn ace_attorney_dialog(
    _: Vec<DecodedImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let ratio = 0.6;

    let shadow_texts_width = |texts: &Vec<ShadowText>| -> f32 {
        texts[0].width() + texts[1..].iter().map(|t| t.width() * ratio).sum::<f32>()
    };

    let shadow_texts_height = |texts: &Vec<ShadowText>| -> f32 {
        texts
            .iter()
            .map(|t| t.height())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    };

    let text = &texts[0];

    let mut font_size = 650.0;
    let mut shadow_width = 20.0;
    let max_width = 900.0;
    let min_font_size = 100.0;
    let shadow_texts = loop {
        let shadow_texts = text
            .chars()
            .map(|c| ShadowText::new(c.to_string(), font_size, shadow_width))
            .collect::<Vec<_>>();
        if shadow_texts_width(&shadow_texts) <= max_width {
            break Ok(shadow_texts);
        }
        if font_size <= min_font_size {
            break Err(Error::TextOverLength(text.clone()));
        }
        font_size -= 10.0;
        shadow_width -= 0.3;
    }?;

    let frame = load_image("ace_attorney_dialog/bubble.png")?;
    let mark = load_image("ace_attorney_dialog/mark.png")?;

    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.rotate(
        -10.0,
        Some(Point::new(
            frame.width() as f32 / 2.0,
            frame.height() as f32 / 2.0,
        )),
    );
    let mut x = (frame.width() as f32 - shadow_texts_width(&shadow_texts)) / 2.0;
    let y = (frame.height() as f32 - shadow_texts_height(&shadow_texts)) / 2.0;
    for text in &shadow_texts {
        text.draw_on_canvas(canvas, (x, y));
        x += text.width() * ratio;
    }
    let mark_height = font_size * 1.2;
    let mark_y = y + shadow_texts_height(&shadow_texts) - mark_height;
    let mark = mark.resize_height(mark_height as i32);
    canvas.draw_image(&mark, (x, mark_y), None);

    encode_png(surface.image_snapshot())
}

register_meme!(
    "ace_attorney_dialog",
    ace_attorney_dialog,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["表情包制作"],
    keywords = &["逆转裁判气泡"],
    date_created = local_date(2024, 5, 3),
    date_modified = local_date(2024, 12, 18),
);
