use std::fs::read;

use chrono::{DateTime, Local, TimeZone};
use qrcode::{render::Pixel, EcLevel, QrCode, Version};
use skia_safe::{
    scalar, surfaces,
    textlayout::{Decoration, TextDecoration, TextDecorationMode},
    Color, Color4f, Data, FilterMode, IRect, ISize, Image, MipmapMode, Paint, PaintJoin,
    PaintStyle, SamplingOptions, Surface,
};

use crate::{config::MEME_HOME, error::Error};

pub(crate) fn new_surface(size: impl Into<ISize>) -> Surface {
    surfaces::raster_n32_premul(size).unwrap()
}

pub(crate) fn new_paint(color: impl Into<Color4f>) -> Paint {
    let mut paint = Paint::new(color.into(), None);
    paint.set_anti_alias(true);
    paint
}

pub(crate) fn new_stroke_paint(color: impl Into<Color4f>, stroke_width: scalar) -> Paint {
    let mut paint = Paint::new(color.into(), None);
    paint.set_anti_alias(true);
    paint.set_stroke_width(stroke_width);
    paint.set_style(PaintStyle::Stroke);
    paint.set_stroke_join(PaintJoin::Round);
    paint
}

pub(crate) fn new_decoration(text_decoration: TextDecoration, color: Color) -> Decoration {
    let mut decoration = Decoration::default();
    decoration.ty = text_decoration;
    decoration.mode = TextDecorationMode::Through;
    decoration.color = color;
    decoration.thickness_multiplier = 1.5;
    decoration
}

pub(crate) fn color_from_hex_code(hex_code: &str) -> Color {
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

pub(crate) fn default_sampling_options() -> SamplingOptions {
    SamplingOptions::new(FilterMode::Linear, MipmapMode::Linear)
}

pub(crate) fn local_date(year: i32, month: u32, day: u32) -> DateTime<Local> {
    Local.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()
}

pub(crate) fn load_image(path: impl Into<String>) -> Result<Image, Error> {
    let image_path = MEME_HOME.join("resources/images").join(path.into());
    let data = Data::new_copy(&read(image_path)?);
    Image::from_encoded(data).ok_or(Error::ImageDecodeError("Skia decode error".to_string()))
}

pub(crate) fn grid_pattern_image() -> Image {
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

#[derive(Clone, Copy)]
struct SkiaPixel {
    color: Color,
}

impl Pixel for SkiaPixel {
    type Image = Image;
    type Canvas = SkiaCanvas;

    fn default_unit_size() -> (u32, u32) {
        (8, 8)
    }

    fn default_color(color: qrcode::Color) -> Self {
        match color {
            qrcode::Color::Dark => Self {
                color: Color::BLACK,
            },
            qrcode::Color::Light => Self {
                color: Color::WHITE,
            },
        }
    }
}

struct SkiaCanvas {
    surface: Surface,
    dark_paint: Paint,
}

impl qrcode::render::Canvas for SkiaCanvas {
    type Pixel = SkiaPixel;
    type Image = Image;

    fn new(width: u32, height: u32, dark_pixel: Self::Pixel, light_pixel: Self::Pixel) -> Self {
        let mut surface = new_surface((width as i32, height as i32));
        let canvas = surface.canvas();
        canvas.clear(light_pixel.color);
        let mut dark_paint = Paint::default();
        dark_paint.set_color(dark_pixel.color);
        dark_paint.set_anti_alias(false);
        Self {
            surface,
            dark_paint,
        }
    }

    fn draw_dark_pixel(&mut self, x: u32, y: u32) {
        let canvas = self.surface.canvas();
        canvas.draw_point((x as i32, y as i32), &self.dark_paint);
    }

    fn into_image(self) -> Self::Image {
        let mut surface = self.surface.clone();
        surface.image_snapshot()
    }
}

pub(crate) fn qrcode_image(message: &str) -> Image {
    let qrcode = QrCode::with_version(message, Version::Normal(5), EcLevel::Q).unwrap();
    qrcode.render::<SkiaPixel>().quiet_zone(false).build()
}
