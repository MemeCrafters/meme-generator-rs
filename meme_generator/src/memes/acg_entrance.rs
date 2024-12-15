use skia_safe::{Color, IRect, Image};

use crate::{
    error::Error,
    manager::register_meme,
    meme::DecodedImage,
    utils::{
        canvas::CanvasExt,
        encoder::make_png_or_gif,
        image::{Fit, ImageExt},
        load_image, local_date, new_paint, new_surface,
        options::NoOptions,
        text::TextParams,
    },
};

const DEFAULT_TEXT: &str = "走，跟我去二次元吧";

fn acg_entrance(
    images: &mut Vec<DecodedImage>,
    texts: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };
    let bg = load_image("acg_entrance/0.png")?;
    let mut surface = bg.to_surface();
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(30, 720, bg.width() - 30, 810),
        text,
        25.0,
        50.0,
        TextParams {
            paint: new_paint(Color::WHITE),
            ..Default::default()
        },
    )?;
    let bg = surface.image_snapshot();

    let func = |images: &Vec<Image>| {
        let mut surface = new_surface(bg.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].resize_fit((290, 410), Fit::Cover);
        canvas.draw_image(&image, (190.0, 265.0), None);
        canvas.draw_image(&bg, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "acg_entrance",
    acg_entrance,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["二次元入口"],
    date_created = local_date(2023, 3, 30),
    date_modified = local_date(2023, 3, 30),
);
