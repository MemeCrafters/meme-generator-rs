use rand::{Rng, seq::SliceRandom};
use skia_safe::{Canvas, FontStyle, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::encode_png,
    image::ImageExt,
    text::Text2Image,
    text_params,
    tools::{color_from_hex_code, load_image, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

struct BoxChar {
    image: Image,
    width: i32,
    outer_width: i32,
    outer_height: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CharMode {
    First,
    White,
    Red,
}

const COLOR_RED: &str = "#E5191C";
const COLOR_WHITE: &str = "#FDFDFD";
const COLOR_BLACK: &str = "#0F0F0F";

impl BoxChar {
    fn new(char: char, mode: CharMode, font_size: f32) -> Self {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(-10.0..0.0);
        let angle = match mode {
            CharMode::First => angle,
            _ => angle * [-1.0, 1.0].choose(&mut rng).unwrap(),
        };
        let scale = match mode {
            CharMode::First => 1.1,
            _ => 1.0 - rng.gen_range(0..3) as f32 / 10.0,
        };
        let font_size = font_size * scale;
        let color = match mode {
            CharMode::Red => color_from_hex_code(COLOR_RED),
            _ => color_from_hex_code(COLOR_WHITE),
        };
        let text2image = Text2Image::from_text(
            char.to_string(),
            font_size,
            text_params!(paint = new_paint(color), font_style = FontStyle::bold()),
        );

        let bg_w_scale = 1.4;
        let bg_h_scale = 1.1;
        let bg_color = match mode {
            CharMode::First => color_from_hex_code(COLOR_RED),
            _ => color_from_hex_code(COLOR_BLACK),
        };

        let text_w = text2image.longest_line();
        let text_h = text2image.height();
        let bg_w: f32 = text_w * bg_w_scale;
        let bg_h: f32 = text_h * bg_h_scale;

        let mut surface = new_surface((bg_w as i32, bg_h as i32));
        let canvas = surface.canvas();
        canvas.clear(bg_color);
        let bg = surface.image_snapshot();

        let bg = if mode == CharMode::First {
            let extra_bg_scale = 1.2;
            let extra_bg_w = (bg_w * extra_bg_scale) as i32;
            let extra_bg_h = (bg_h * extra_bg_scale) as i32;
            let mut surface = new_surface((extra_bg_w, extra_bg_h));
            let canvas = surface.canvas();
            canvas.clear(color_from_hex_code(COLOR_BLACK));
            let extra_angle = rng.gen_range(0.0..5.0) * [-1.0, 1.0].choose(&mut rng).unwrap();
            let bg = bg.rotate(extra_angle);
            canvas.draw_image(
                &bg,
                (
                    (extra_bg_w - bg.width()) / 2,
                    (extra_bg_h - bg.height()) / 2,
                ),
                None,
            );
            surface.image_snapshot()
        } else {
            bg
        };

        let border_size = 6;
        let border_w = bg.width() + border_size * 2;
        let border_h = bg.height() + border_size * 2;
        let mut surface = new_surface((border_w, border_h));
        let canvas = surface.canvas();
        canvas.clear(color_from_hex_code(COLOR_WHITE));
        canvas.draw_image(&bg, (border_size, border_size), None);
        text2image.draw_on_canvas(
            &canvas,
            (
                (border_w as f32 - text2image.longest_line()) / 2.0,
                (border_h as f32 - text2image.height()) / 2.0,
            ),
        );
        let border = surface.image_snapshot();
        let image = border.rotate(angle);

        let width = border.width();
        let outer_width = image.width();
        let outer_height = image.height();

        BoxChar {
            image,
            width,
            outer_width,
            outer_height,
        }
    }
}

struct BoxLine {
    box_chars: Vec<BoxChar>,
    width: i32,
    height: i32,
}

impl BoxLine {
    fn new(box_chars: Vec<BoxChar>) -> Self {
        let mut width: i32 = box_chars.iter().map(|bc| bc.width).sum();
        if let Some(first_char) = box_chars.first() {
            width += (first_char.outer_width - first_char.width) / 2;
        }
        if let Some(last_char) = box_chars.last() {
            width += (last_char.outer_width - last_char.width) / 2;
        }
        let height = box_chars
            .iter()
            .map(|bc| bc.outer_height)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        BoxLine {
            box_chars,
            width,
            height,
        }
    }

    fn draw_on_canvas(&self, canvas: &Canvas, x: i32, y: i32) {
        let mut x = x;
        if let Some(first_char) = self.box_chars.first() {
            x += (first_char.outer_width - first_char.width) / 2;
        }
        for box_char in &self.box_chars {
            canvas.draw_image(
                &box_char.image,
                (
                    x - (box_char.outer_width - box_char.width) / 2,
                    y + (self.height - box_char.outer_height) / 2,
                ),
                None,
            );
            x += box_char.width;
        }
    }
}

fn p5letter(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let lines: Vec<&str> = text.lines().collect();
    if lines.len() > 5 {
        return Err(Error::TextOverLength(text.to_string()));
    }

    let mut box_lines: Vec<BoxLine> = Vec::new();
    for line in lines {
        let mut box_chars: Vec<BoxChar> = Vec::new();
        for c in line.chars() {
            if c.is_whitespace() {
                continue;
            }
            let mode = if box_chars.is_empty() {
                CharMode::First
            } else {
                if rand::thread_rng().gen_range(0.0..1.0) < 0.4 {
                    CharMode::Red
                } else {
                    CharMode::White
                }
            };
            let box_char = BoxChar::new(c, mode, 120.0);
            box_chars.push(box_char);
        }
        if box_chars.is_empty() {
            continue;
        }
        let line = BoxLine::new(box_chars);
        if line.width > 1700 {
            return Err(Error::TextOverLength(text.to_string()));
        }
        box_lines.push(line);
    }

    let frame = load_image("p5letter/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();

    let total_height: i32 = box_lines.iter().map(|line| line.height).sum();
    let mut y = (frame.height() - total_height) / 2;
    for line in box_lines {
        line.draw_on_canvas(&canvas, (frame.width() - line.width) / 2, y);
        y += line.height;
    }

    encode_png(surface.image_snapshot())
}

register_meme!(
    "p5letter",
    p5letter,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["TAKEYOURHEART"],
    keywords = &["p5预告信"],
    tags = MemeTags::persona5(),
    date_created = local_date(2024, 11, 13),
    date_modified = local_date(2024, 11, 13),
);
