use skia_safe::{IRect, Image};

use crate::{
    error::Error,
    manager::register_meme,
    meme::{shortcut, DecodedImage},
    utils::{
        color_from_hex_code,
        encoder::make_png_or_gif,
        image::{Fit, ImageExt},
        load_image, local_date, new_paint, new_surface,
        options::NoOptions,
        text::{Text2Image, TextParams},
    },
};

fn steam_message(
    images: &mut Vec<DecodedImage>,
    texts: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let mut name = images[0].name.as_str();
    if name.is_empty() {
        name = "好友";
    }
    let game = texts[0].clone();

    let text_name = Text2Image::from_text(
        name,
        65.0,
        TextParams {
            paint: new_paint(color_from_hex_code("#e3ffc2")),
            ..Default::default()
        },
    );
    let text_play = Text2Image::from_text(
        "正在玩",
        62.0,
        TextParams {
            paint: new_paint(color_from_hex_code("#d1d1c0")),
            ..Default::default()
        },
    );
    let text_game = Text2Image::from_text(
        game,
        65.0,
        TextParams {
            paint: new_paint(color_from_hex_code("#91c257")),
            ..Default::default()
        },
    );

    let avatar_w = 280;
    let padding_h = 50;
    let padding_v = 80;
    let margin_rec = 6;
    let rec_w = 15;
    let margin_text = 80;
    let text_w = text_name
        .longest_line()
        .max(text_play.longest_line())
        .max(text_game.longest_line())
        .max(1300.0) as i32;
    let text_x = padding_h + avatar_w + margin_rec + rec_w + margin_text;
    let frame_w = text_x + text_w + padding_h;
    let frame_h = padding_v * 2 + avatar_w;

    let mut surface = new_surface((frame_w, frame_h));
    let canvas = surface.canvas();
    canvas.clear(color_from_hex_code("#14161f"));
    canvas.draw_irect(
        IRect::from_xywh(padding_h, padding_v, avatar_w, avatar_w),
        &new_paint(color_from_hex_code("#191b23")),
    );
    let rec_x = padding_h + avatar_w + margin_rec;
    canvas.draw_irect(
        IRect::from_xywh(rec_x, padding_v, rec_w, frame_h - padding_v * 2),
        &new_paint(color_from_hex_code("#6cbe48")),
    );
    let logo = load_image("steam_message/logo.png")?;
    canvas.draw_image(&logo, (frame_w - 870, -370), None);
    text_play.draw_on_canvas(
        canvas,
        (text_x as f32, (frame_h as f32 - text_play.height()) / 2.0),
    );
    text_name.draw_on_canvas(
        canvas,
        (
            text_x as f32,
            padding_v as f32 + 40.0 - text_name.height() / 2.0,
        ),
    );
    text_game.draw_on_canvas(
        canvas,
        (
            text_x as f32,
            frame_h as f32 - padding_v as f32 - 40.0 - text_game.height() / 2.0,
        ),
    );

    let func = |images: &Vec<Image>| {
        let avatar = images[0].resize_fit((avatar_w, avatar_w), Fit::Cover);
        let mut surface = surface.clone();
        let canvas = surface.canvas();
        canvas.draw_image(&avatar, (padding_h as f32, padding_v as f32), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "steam_message",
    steam_message,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["黑神话：悟空"],
    keywords = &["steam消息"],
    shortcuts = &[shortcut!(
        r"(?P<name>\S+)正在玩(?P<game>\S+)",
        humanized = "xx正在玩xx",
        names = &["${name}"],
        texts = &["${game}"],
    )],
    date_created = local_date(2024, 8, 21),
    date_modified = local_date(2024, 8, 21),
);
