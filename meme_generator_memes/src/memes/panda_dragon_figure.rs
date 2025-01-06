use skia_safe::{Color, IRect};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{MemeOptions, NamedImage},
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    shortcut, text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::register_meme;

#[derive(MemeOptions)]
struct Name {
    /// 龙图名字
    #[option(short, long, default = "责怪龙")]
    pub name: Option<String>,
}

fn panda_dragon_figure(
    _: Vec<NamedImage>,
    texts: Vec<String>,
    options: Name,
) -> Result<Vec<u8>, Error> {
    let name = options.name.unwrap();
    let text = &texts[0];
    let frame = load_image("panda_dragon_figure/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(0, 470, 470, 550),
        name,
        20.0,
        60.0,
        text_params!(paint = new_paint(Color::WHITE)),
    )?;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(0, 0, 550, 120),
        text,
        20.0,
        100.0,
        None,
    )?;
    encode_png(surface.image_snapshot())
}

register_meme! {
    "panda_dragon_figure",
    panda_dragon_figure,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["我要玩原神"],
    keywords = &["熊猫龙图"],
    shortcuts = &[shortcut!(
        r"(?P<name>\S{1,10})龙[\s:：]+(?P<text>\S+)",
        texts= &["${text}"],
        options= &[("name", "${name}龙")],
        humanized= "xx龙：xx",
    )],
    date_created = local_date(2024, 10, 30),
    date_modified = local_date(2024, 10, 30),
}
