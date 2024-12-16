use image::Rgba;
use qrcode::{EcLevel, QrCode, Version};
use skia_safe::{images, AlphaType, Color, ColorType, Data, IRect, ISize, Image, ImageInfo};

use crate::utils::{color_from_hex_code, new_paint, new_surface};

pub(crate) fn empty_image() -> Image {
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

pub(crate) fn qrcode_image(message: &str) -> Image {
    let qr = QrCode::with_version(message, Version::Normal(5), EcLevel::Q).unwrap();
    let qr_image = qr.render::<Rgba<u8>>().quiet_zone(false).build();
    let image_info = ImageInfo::new(
        (qr_image.width() as i32, qr_image.height() as i32),
        ColorType::RGBA8888,
        AlphaType::Premul,
        None,
    );
    let data = Data::new_copy(qr_image.as_raw());
    images::raster_from_data(&image_info, data, qr_image.width() as usize * 4).unwrap()
}
