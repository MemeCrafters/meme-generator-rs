use skia_safe::{Color, FontStyle, IRect, RRect, Rect};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    canvas::CanvasExt,
    encoder::encode_png,
    text_params,
    tools::{local_date, new_paint, new_stroke_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn yeshu(_: Vec<NamedImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let color_yellow = Color::from_rgb(254, 239, 80);
    let color_blue = Color::from_rgb(0, 33, 252);
    let color_red = Color::from_rgb(253, 0, 0);

    let paint_black = new_paint(Color::BLACK);
    let paint_white = new_paint(Color::WHITE);
    let paint_yellow = new_paint(color_yellow);
    let paint_blue = new_paint(color_blue);
    let paint_red = new_paint(color_red);

    let frame_w = 600;
    let frame_h = 1200;
    let mut surface = new_surface((frame_w, frame_h));
    let canvas = surface.canvas();
    canvas.clear(color_yellow);

    let padding = 4;
    canvas.draw_irect(
        IRect::from_ltrb(padding, padding, frame_w - padding, frame_h - padding),
        &paint_black,
    );

    let padding_x = 28;
    let h1 = 16;
    let h2 = 1000;
    canvas.draw_irect(
        IRect::from_ltrb(padding_x, h1, frame_w - padding_x, h2),
        &paint_yellow,
    );

    let padding_x = 34;
    let h1 = 88;
    let h2 = 999;
    canvas.draw_irect(
        IRect::from_ltrb(padding_x, h1, frame_w - padding_x, h2),
        &paint_black,
    );

    let padding_x = padding_x + 2;
    let h1 = h1 + 2;
    let h2 = h2 - 2;
    canvas.draw_irect(
        IRect::from_ltrb(padding_x, h1, frame_w - padding_x, h2),
        &paint_blue,
    );

    let h1 = 16;
    let h2 = 88;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(padding_x, h1, frame_w - padding_x, h2),
        &texts[0],
        40.0,
        70.0,
        text_params!(paint = paint_black.clone(), font_style = FontStyle::bold()),
    )?;

    let h1 = 90;
    let h2 = 170;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(padding_x, h1, frame_w - padding_x, h2),
        &texts[1],
        40.0,
        70.0,
        text_params!(paint = paint_white.clone(), font_style = FontStyle::bold()),
    )?;

    let h1 = 390;
    let h2 = 590;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(padding_x, h1, frame_w - padding_x, h2),
        &texts[3],
        100.0,
        200.0,
        text_params!(
            paint = paint_white.clone(),
            stroke_paint = new_stroke_paint(Color::BLACK, 10.0),
            font_style = FontStyle::bold()
        ),
    )?;

    let h1 = 580;
    let h2 = 660;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(padding_x, h1, frame_w - padding_x, h2),
        &texts[4],
        40.0,
        70.0,
        text_params!(paint = paint_yellow.clone(), font_style = FontStyle::bold()),
    )?;

    let h1 = 740;
    let h2 = 1000;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(padding_x, h1, frame_w - padding_x, h2),
        &texts[6],
        150.0,
        250.0,
        text_params!(
            paint = new_paint(color_yellow),
            stroke_paint = new_stroke_paint(Color::BLACK, 12.0),
            font_style = FontStyle::bold()
        ),
    )?;

    let padding_x = 38;
    let h1 = 170;
    let h2 = 403;
    canvas.draw_rrect(
        &RRect::new_rect_xy(
            Rect::from_irect(IRect::from_xywh(
                padding_x,
                h1,
                frame_w - padding_x * 2,
                h2 - h1,
            )),
            30.0,
            30.0,
        ),
        &paint_yellow,
    );

    let padding_x = padding_x + 3;
    let h1 = h1 + 3;
    let h2 = h2 - 3;
    canvas.draw_rrect(
        &RRect::new_rect_xy(
            Rect::from_irect(IRect::from_xywh(
                padding_x,
                h1,
                frame_w - padding_x * 2,
                h2 - h1,
            )),
            28.0,
            28.0,
        ),
        &paint_red,
    );

    let padding_x = padding_x + 10;
    let h1 = h1 - 15;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(padding_x, h1, frame_w - padding_x, h2),
        &texts[2],
        100.0,
        230.0,
        text_params!(paint = paint_white.clone(), font_style = FontStyle::bold()),
    )?;

    let padding_x = 32;
    let h1 = 668;
    let h2 = 760;
    canvas.draw_rrect(
        &RRect::new_rect_xy(
            Rect::from_irect(IRect::from_xywh(
                padding_x,
                h1,
                frame_w - padding_x * 2,
                h2 - h1,
            )),
            5.0,
            5.0,
        ),
        &paint_red,
    );

    let padding_x = padding_x + 4;
    let h1 = h1 + 4;
    let h2 = h2 - 4;
    canvas.draw_rrect(
        &RRect::new_rect_xy(
            Rect::from_irect(IRect::from_xywh(
                padding_x,
                h1,
                frame_w - padding_x * 2,
                h2 - h1,
            )),
            1.0,
            1.0,
        ),
        &paint_yellow,
    );

    let h1 = h1 - 5;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(padding_x, h1, frame_w - padding_x, h2),
        &texts[5],
        50.0,
        100.0,
        text_params!(paint = paint_red.clone(), font_style = FontStyle::bold()),
    )?;

    let padding_x = 10;
    let h1 = 1000;
    let h2 = 1188;
    canvas.draw_rrect(
        &RRect::new_rect_xy(
            Rect::from_irect(IRect::from_xywh(
                padding_x,
                h1,
                frame_w - padding_x * 2,
                h2 - h1,
            )),
            8.0,
            8.0,
        ),
        &paint_yellow,
    );

    let padding_x = padding_x + 2;
    let h1 = h1 + 2;
    let h2 = h2 - 2;
    canvas.draw_rrect(
        &RRect::new_rect_xy(
            Rect::from_irect(IRect::from_xywh(
                padding_x,
                h1,
                frame_w - padding_x * 2,
                h2 - h1,
            )),
            6.0,
            6.0,
        ),
        &paint_red,
    );

    let padding_x = padding_x + 10;
    let h1 = h1 - 10;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(padding_x, h1, frame_w - padding_x, h2),
        &texts[7],
        50.0,
        180.0,
        text_params!(paint = paint_white.clone(), font_style = FontStyle::bold()),
    )?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "yeshu",
    yeshu,
    min_texts = 8,
    max_texts = 8,
    default_texts = &[
        "椰子特产在海南",
        "正宗",
        "椰树",
        "29年",
        "坚持在海南岛",
        "用新鲜椰子肉",
        "鲜榨",
        "不用椰浆\n不加香精当生榨",
    ],
    keywords = &["椰树椰汁"],
    date_created = local_date(2024, 11, 5),
    date_modified = local_date(2024, 11, 5),
);
