use rand::seq::SliceRandom;
use skia_safe::IRect;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    shortcut,
    tools::{load_image, local_date},
};

use crate::{register_meme, tags::MemeTags, union_tags};

#[derive(MemeOptions)]
struct Position {
    /// 角色名
    #[option(short, long, choices = ["arisu", "izuna", "key", "kokona", "mari", "sena", "yuuka"])]
    character: Option<String>,

    /// 消息框的位置
    #[option(short, long, choices=["left", "right"])]
    position: Option<String>,

    /// 左
    #[option(short, long, short_aliases=['左'], default=false)]
    left: Option<bool>,

    /// 右
    #[option(short, long, short_aliases=['右'], default=false)]
    right: Option<bool>,
}

fn ba_say(_: Vec<InputImage>, texts: Vec<String>, options: Position) -> Result<Vec<u8>, Error> {
    let character = options.character.as_deref().unwrap_or({
        let mut rng = rand::thread_rng();
        ["arisu", "izuna", "key", "kokona", "mari", "sena", "yuuka"]
            .choose(&mut rng)
            .unwrap()
    });
    let position = if options.left.unwrap() {
        "left"
    } else if options.right.unwrap() {
        "right"
    } else {
        options.position.as_deref().unwrap_or({
            let mut rng = rand::thread_rng();
            ["left", "right"].choose(&mut rng).unwrap()
        })
    };
    let text = &texts[0];

    let mut frame = load_image(format!("ba_say/{character}.png"))?;
    if position == "left" {
        frame = frame.flip_horizontal();
    }
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    let rect = if position == "left" {
        IRect::from_ltrb(60, 0, 580, 200)
    } else {
        IRect::from_ltrb(500, 0, 1020, 200)
    };
    canvas.draw_text_area_auto_font_size(rect, text, 20.0, 100.0, None)?;

    encode_png(surface.image_snapshot())
}

register_meme!(
    "ba_say",
    ba_say,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["那我问你"],
    keywords = &["ba说"],
    shortcuts = &[
        shortcut!("爱丽丝说", options = &[("character", "arisu")]),
        shortcut!("泉奈说", options = &[("character", "izuna")]),
        shortcut!("key说", options = &[("character", "key")]),
        shortcut!("心奈说", options = &[("character", "kokona")]),
        shortcut!("玛丽说", options = &[("character", "mari")]),
        shortcut!("濑名说", options = &[("character", "sena")]),
        shortcut!("优香说", options = &[("character", "yuuka")]),
    ],
    tags = union_tags!(
        MemeTags::arisu(),
        MemeTags::izuna(),
        MemeTags::key(),
        MemeTags::kokona(),
        MemeTags::mari(),
        MemeTags::sena(),
        MemeTags::yuuka(),
    ),
    date_created = local_date(2024, 12, 12),
    date_modified = local_date(2024, 12, 12),
);
