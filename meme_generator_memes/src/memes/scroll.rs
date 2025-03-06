use skia_safe::{Color, IRect, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    text::Text2Image,
    text_params,
    tools::{color_from_hex_code, load_image, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn scroll(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];

    let mut text2image =
        Text2Image::from_text(text, 40.0, text_params!(text_align = TextAlign::Left));
    if text2image.longest_line() > 600.0 {
        text2image.layout(600.0);
    }
    if text2image.height() > 200.0 {
        return Err(Error::TextOverLength(text.to_string()));
    }
    let text_w = text2image.longest_line().ceil() as i32;
    let text_h = text2image.height().ceil() as i32;

    let box_w = text_w + 140;
    let box_h = (text_h + 103).max(150);
    let mut box_surface = new_surface((box_w, box_h));
    let canvas = box_surface.canvas();
    canvas.clear(color_from_hex_code("#eaedf4"));
    let corner1 = load_image("scroll/corner1.png")?;
    let corner2 = load_image("scroll/corner2.png")?;
    let corner3 = load_image("scroll/corner3.png")?;
    let corner4 = load_image("scroll/corner4.png")?;
    canvas.draw_image(&corner1, (0, 0), None);
    canvas.draw_image(&corner2, (0, box_h - 75), None);
    canvas.draw_image(&corner3, (text_w + 70, 0), None);
    canvas.draw_image(&corner4, (text_w + 70, box_h - 75), None);
    canvas.draw_irect(
        IRect::from_xywh(70, 20, text_w, box_h - 40),
        &new_paint(Color::WHITE),
    );
    canvas.draw_irect(
        IRect::from_xywh(27, 75, text_w + 88, box_h - 150),
        &new_paint(Color::WHITE),
    );
    text2image.draw_on_canvas(&canvas, (70, 17 + (box_h - 40 - text_h) / 2));
    let box_image = box_surface.image_snapshot();

    let mut dialog_surface = new_surface((box_w, box_h * 4));
    let canvas = dialog_surface.canvas();
    canvas.clear(color_from_hex_code("#eaedf4"));
    for i in 0..4 {
        canvas.draw_image(&box_image, (0, box_h * i), None);
    }
    let dialog = dialog_surface.image_snapshot();

    let mut encoder = GifEncoder::new();
    let num = 30;
    let dy = dialog.height() / num;
    for i in 0..num {
        let mut frame = new_surface(dialog.dimensions());
        let frame_canvas = frame.canvas();
        frame_canvas.draw_image(&dialog, (0, -dy * i), None);
        frame_canvas.draw_image(&dialog, (0, dialog.height() - dy * i), None);
        encoder.add_frame(frame.image_snapshot(), 0.05)?;
    }
    Ok(encoder.finish())
}

register_meme!(
    "scroll",
    scroll,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["你们说话啊"],
    keywords = &["滚屏"],
    date_created = local_date(2022, 1, 19),
    date_modified = local_date(2023, 2, 14),
);
