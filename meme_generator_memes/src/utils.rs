use qrcode::{render::Pixel, EcLevel, QrCode, Version};
use skia_safe::{Color, Image, Paint, Surface};

use meme_generator_utils::tools::new_surface;

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

pub fn qrcode_image(message: &str) -> Image {
    let qrcode = QrCode::with_version(message, Version::Normal(5), EcLevel::Q).unwrap();
    qrcode.render::<SkiaPixel>().quiet_zone(false).build()
}
