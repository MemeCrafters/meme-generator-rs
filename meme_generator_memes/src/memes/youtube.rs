use skia_safe::{Color, IRect, RRect, Rect};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::encode_png,
    image::ImageExt,
    text::Text2Image,
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn youtube(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let left_img = Text2Image::from_text(
        &texts[0],
        200.0,
        text_params!(paint = new_paint(Color::BLACK)),
    );
    let mut right_img = Text2Image::from_text(
        &texts[1],
        200.0,
        text_params!(paint = new_paint(Color::WHITE)),
    );
    let right_w = right_img.longest_line().ceil().max(400.0);
    right_img.layout(right_w);

    let left_w = left_img.longest_line().ceil() as i32;
    let left_h = left_img.height().ceil() as i32;
    let right_w = right_w as i32;
    let right_h = right_img.height().ceil() as i32;

    let left_padding_x = 30;
    let right_padding_x = 50;
    let padding_y = 20;
    let margin_x = 50;
    let margin_y = 50;
    let frame_w = left_w + right_w + left_padding_x * 2 + right_padding_x * 2 + margin_x * 2;
    let frame_h = left_h.max(right_h) + padding_y * 2 + margin_y * 2;

    let mut surface = new_surface((frame_w, frame_h));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);

    let corner = load_image("youtube/corner.png")?;
    let ratio = right_h as f32 / (2.0 * corner.height() as f32);
    let corner = corner.resize_exact((
        (corner.width() as f32 * ratio) as i32,
        (corner.height() as f32 * ratio) as i32,
    ));

    let x0 = margin_x + left_w + left_padding_x * 2;
    let y0 = frame_h - right_h - padding_y * 2 - margin_y;
    let x1 = frame_w - corner.width() - margin_x;
    let y1 = frame_h - corner.height() - margin_y;

    canvas.draw_image(&corner, (x0, y0), None);
    canvas.draw_image(&corner.flip_vertical(), (x0, y1), None);
    canvas.draw_image(&corner.flip_horizontal(), (x1, y0), None);
    canvas.draw_image(&corner.flip_vertical().flip_horizontal(), (x1, y1), None);

    let radius = right_h as f32 / 2.0;
    canvas.draw_rrect(
        &RRect::new_rect_xy(
            Rect::from_irect(IRect::from_xywh(
                x0,
                y0,
                right_w + right_padding_x * 2,
                right_h + padding_y * 2,
            )),
            radius,
            radius,
        ),
        &new_paint(Color::from_rgb(230, 33, 23)),
    );

    left_img.draw_on_canvas(
        canvas,
        (
            margin_x + left_padding_x,
            frame_h - left_h - padding_y - margin_y,
        ),
    );
    right_img.draw_on_canvas(
        canvas,
        (
            x0 + right_padding_x,
            frame_h - right_h - padding_y - margin_y,
        ),
    );

    encode_png(surface.image_snapshot())
}

register_meme!(
    "youtube",
    youtube,
    min_texts = 2,
    max_texts = 2,
    default_texts = &["Porn", "Hub"],
    keywords = &["yt", "youtube"],
    date_created = local_date(2022, 10, 27),
    date_modified = local_date(2023, 2, 14),
);
