use skia_safe::{Color, IRect, ISize, Image};

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
