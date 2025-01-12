use std::fs::read;

use chrono::{DateTime, Local, TimeZone};
use skia_safe::{
    scalar, surfaces,
    textlayout::{Decoration, TextDecoration, TextDecorationMode},
    Color, Color4f, Data, FilterMode, IRect, ISize, Image, MipmapMode, Paint, PaintJoin,
    PaintStyle, SamplingOptions, Surface,
};

use meme_generator_core::error::Error;

use crate::config::IMAGES_DIR;

pub fn new_surface(size: impl Into<ISize>) -> Surface {
    surfaces::raster_n32_premul(size).unwrap()
}

pub fn new_paint(color: impl Into<Color4f>) -> Paint {
    let mut paint = Paint::new(color.into(), None);
    paint.set_anti_alias(true);
    paint
}

pub fn new_stroke_paint(color: impl Into<Color4f>, stroke_width: scalar) -> Paint {
    let mut paint = Paint::new(color.into(), None);
    paint.set_anti_alias(true);
    paint.set_stroke_width(stroke_width);
    paint.set_style(PaintStyle::Stroke);
    paint.set_stroke_join(PaintJoin::Round);
    paint
}

pub fn new_decoration(text_decoration: TextDecoration, color: Color) -> Decoration {
    let mut decoration = Decoration::default();
    decoration.ty = text_decoration;
    decoration.mode = TextDecorationMode::Through;
    decoration.color = color;
    decoration.thickness_multiplier = 1.5;
    decoration
}

pub fn color_from_hex_code(hex_code: &str) -> Color {
    let hex_code = hex_code.trim_start_matches('#');
    let r = u8::from_str_radix(&hex_code[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex_code[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex_code[4..6], 16).unwrap();
    let a = if hex_code.len() == 8 {
        u8::from_str_radix(&hex_code[6..8], 16).unwrap()
    } else {
        255
    };
    Color::from_argb(a, r, g, b)
}

pub fn default_sampling_options() -> SamplingOptions {
    SamplingOptions::new(FilterMode::Linear, MipmapMode::Linear)
}

pub fn local_date(year: i32, month: u32, day: u32) -> DateTime<Local> {
    Local.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()
}

pub fn load_image(path: impl Into<String>) -> Result<Image, Error> {
    let path = path.into();
    let image_path = IMAGES_DIR.join(&path);
    if !(image_path.exists() && image_path.is_file()) {
        return Err(Error::ImageAssetMissing(path));
    }
    let data = Data::new_copy(&read(&image_path).unwrap());
    Image::from_encoded(data).ok_or(Error::ImageDecodeError(format!(
        "Failed to decode image: {}",
        path
    )))
}

pub fn grid_pattern_image() -> Image {
    let mut surface = new_surface(ISize::new(500, 500));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    let paint = new_paint(color_from_hex_code("#cccccc"));
    for x in 0..20 {
        for y in 0..20 {
            if (x + y) % 2 == 0 {
                canvas.draw_irect(IRect::from_xywh(x * 25, y * 25, 25, 25), &paint);
            }
        }
    }
    surface.image_snapshot()
}
