use skia_safe::{Color, FontStyle, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{MemeOptions, NamedImage},
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::register_meme;

const DEFAULT_TEXT: &str = "我是急急国王";

#[derive(MemeOptions)]
struct Options {
    /// 方块中的文字
    #[option(long, default = "急")]
    pub block_text: Option<String>,
}

fn jiji_king(
    images: Vec<NamedImage>,
    texts: Vec<String>,
    options: Options,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };

    let block_num = 5;
    let block_text = options.block_text.unwrap();
    let mut block_surface = new_surface((90, 90));
    let canvas = block_surface.canvas();
    canvas.clear(Color::BLACK);
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(0, 0, 90, 90),
        block_text,
        30.0,
        60.0,
        text_params!(
            paint = new_paint(Color::WHITE),
            font_style = FontStyle::bold()
        ),
    )?;
    let block = block_surface.image_snapshot();

    let frame = load_image("jiji_king/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    for i in 0..block_num {
        canvas.draw_image(&block, (10 + 100 * i, 200), None);
    }
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(10, 300, frame.width() - 10, 390),
        text,
        30.0,
        100.0,
        text_params!(font_style = FontStyle::bold()),
    )?;
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let img = images[0].circle().resize_exact((125, 125));
        canvas.draw_image(&img, (237, 5), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "jiji_king",
    jiji_king,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["急急国王"],
    date_created = local_date(2022, 10, 10),
    date_modified = local_date(2025, 1, 13),
);
