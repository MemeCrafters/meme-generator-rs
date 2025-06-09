use skia_safe::{Color, FontStyle, IRect, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{color_from_hex_code, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn google_captcha(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let name = &images[0].name;

    let mut surface = new_surface((1004, 1539));
    let canvas = surface.canvas();
    canvas.clear(color_from_hex_code("#D5D5D5"));
    canvas.draw_irect(IRect::from_xywh(2, 2, 1000, 1535), &new_paint(Color::WHITE));

    let text_params = text_params!(
        paint = new_paint(Color::WHITE),
        font_style = FontStyle::bold(),
        text_align = TextAlign::Left
    );

    canvas.draw_irect(
        IRect::from_xywh(21, 21, 962, 332),
        &new_paint(color_from_hex_code("#4790E4")),
    );
    canvas
        .draw_text_area(
            IRect::from_ltrb(91, 81, 921, 141),
            "请选择包含",
            40.0,
            text_params.clone(),
        )
        .unwrap();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(91, 141, 921, 231),
        name,
        20.0,
        80.0,
        text_params.clone(),
    )?;
    canvas
        .draw_text_area(
            IRect::from_ltrb(91, 231, 921, 291),
            "的所有图块，如果没有，请点击“跳过”",
            40.0,
            text_params.clone(),
        )
        .unwrap();

    canvas.draw_irect(
        IRect::from_xywh(2, 1355, 1002, 182),
        &new_paint(color_from_hex_code("#D5D5D5")),
    );
    canvas.draw_irect(
        IRect::from_xywh(2, 1357, 1000, 182),
        &new_paint(Color::WHITE),
    );
    canvas.draw_irect(
        IRect::from_xywh(689, 1387, 283, 121),
        &new_paint(color_from_hex_code("#4790E4")),
    );
    canvas
        .draw_text_area(
            IRect::from_xywh(689, 1387, 283, 121),
            "跳过",
            40.0,
            text_params!(
                paint = new_paint(Color::WHITE),
                font_style = FontStyle::bold(),
            ),
        )
        .unwrap();

    let image = images[0].image.square().resize_exact((932, 932));
    let length = 233;
    for i in 0..4 {
        for j in 0..4 {
            canvas.draw_image(
                &image.crop(IRect::from_xywh(233 * i, 233 * j, length, length)),
                (21 + i * (233 + 10), 372 + j * (233 + 10)),
                None,
            );
        }
    }
    encode_png(surface.image_snapshot())
}

register_meme!(
    "google_captcha",
    google_captcha,
    min_images = 1,
    max_images = 1,
    keywords = &["谷歌验证码"],
    date_created = local_date(2024, 8, 15),
    date_modified = local_date(2024, 8, 15),
);
