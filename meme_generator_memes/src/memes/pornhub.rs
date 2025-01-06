use skia_safe::{Color, FontStyle, IRect, RRect, Rect};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::encode_png,
    text::Text2Image,
    text_params,
    tools::{local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn pornhub(_: Vec<NamedImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let left_img = Text2Image::from_text(
        &texts[0],
        200.0,
        text_params!(paint = new_paint(Color::WHITE)),
    );
    let left_w = left_img.longest_line().ceil() as i32;
    let left_h = left_img.height().ceil() as i32;

    let right_img = Text2Image::from_text(
        &texts[1],
        200.0,
        text_params!(
            paint = new_paint(Color::BLACK),
            font_style = FontStyle::bold()
        ),
    );
    let right_w = right_img.longest_line().ceil() as i32;
    let right_h = right_img.height().ceil() as i32;

    let padding_x = 20;
    let padding_y = 10;
    let margin_x = 50;
    let margin_y = 50;
    let frame_w = left_w + right_w + padding_x * 4 + margin_x * 2;
    let frame_h = left_h.max(right_h) + padding_y * 2 + margin_y * 2;

    let mut surface = new_surface((frame_w, frame_h));
    let canvas = surface.canvas();
    canvas.clear(Color::BLACK);
    left_img.draw_on_canvas(
        &canvas,
        (
            margin_x + padding_x,
            margin_y + padding_y + left_h.max(right_h) - left_h,
        ),
    );
    canvas.draw_rrect(
        &RRect::new_rect_xy(
            Rect::from_irect(IRect::from_xywh(
                margin_x + left_w + padding_x * 2,
                margin_y,
                right_w + padding_x * 2,
                right_h + padding_y * 2,
            )),
            20.0,
            20.0,
        ),
        &new_paint(Color::from_rgb(247, 152, 23)),
    );
    right_img.draw_on_canvas(
        &canvas,
        (
            margin_x + left_w + padding_x * 3,
            margin_y + padding_y + left_h.max(right_h) - right_h,
        ),
    );

    encode_png(surface.image_snapshot())
}

register_meme!(
    "pornhub",
    pornhub,
    min_texts = 2,
    max_texts = 2,
    default_texts = &["You", "Tube"],
    keywords = &["ph", "pornhub"],
    date_created = local_date(2022, 10, 27),
    date_modified = local_date(2023, 2, 14),
);
