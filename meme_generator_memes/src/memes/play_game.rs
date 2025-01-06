use skia_safe::{Color, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    text_params,
    tools::{load_image, local_date, new_stroke_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

const DEFAULT_TEXT: &str = "来玩休闲游戏啊";

fn play_game(images: Vec<NamedImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };
    let frame = load_image("play_game/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(20, frame.height() - 70, frame.width() - 20, frame.height()),
        text,
        25.0,
        40.0,
        text_params!(stroke_paint = new_stroke_paint(Color::WHITE, 4.0)),
    )?;
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let screen = images[0].resize_fit((230, 160), Fit::Cover);
        let screen = screen.perspective(&[(0, 5), (227, 0), (216, 150), (0, 165)]);
        canvas.draw_image(&screen.rotate(-9.0), (161, 117), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "play_game",
    play_game,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["玩游戏"],
    date_created = local_date(2022, 1, 4),
    date_modified = local_date(2023, 2, 14),
);
