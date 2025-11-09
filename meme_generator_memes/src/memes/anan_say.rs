use rand::seq::SliceRandom;
use skia_safe::{Color, FontStyle, IRect};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::{register_meme, tags::MemeTags};

#[derive(MemeOptions)]
struct Expression {
    /// 表情类型
    #[option(short, long, choices=["angry", "black", "happy", "shy", "speechless"])]
    expression: Option<String>,
}

fn anan_say(_: Vec<InputImage>, texts: Vec<String>, options: Expression) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let expression = options.expression.as_deref().unwrap_or({
        let mut rng = rand::thread_rng();
        ["angry", "black", "happy", "shy", "speechless"]
            .choose(&mut rng)
            .unwrap()
    });

    let base_image = load_image(&format!("anan/{}.png", expression))?;
    let mut surface = base_image.to_surface();
    let canvas = surface.canvas();

    let text_rect = IRect::from_ltrb(105, 445, 412, 625);
    canvas.draw_text_area_auto_font_size(
        text_rect,
        text,
        20.0,
        60.0,
        text_params!(
            paint = new_paint(Color::BLACK),
            font_style = FontStyle::bold()
        ),
    )?;

    let hand_image = load_image("anan/hand.png")?;
    canvas.draw_image(&hand_image, (0, 0), None);

    encode_png(surface.image_snapshot())
}

register_meme!(
    "anan_say",
    anan_say,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["吾辈很开心"],
    keywords = &["安安说"],
    tags = MemeTags::natsume_anan(),
    date_created = local_date(2025, 11, 8),
    date_modified = local_date(2025, 11, 8),
);
