use skia_safe::{textlayout::TextAlign, Color, IRect, Point};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    canvas::CanvasExt,
    encoder::encode_png,
    image::ImageExt,
    text::Text2Image,
    text_params,
    tools::{load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme};

fn nokia(_: Vec<NamedImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let frame = load_image("nokia/0.jpg")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.rotate(9.3, Some(Point::new(260.0, 300.0)));
    canvas.draw_text_area(
        IRect::from_xywh(270, 210, 730, 90),
        &format!("{}/900", text.chars().count()),
        70.0,
        text_params!(
            font_families = &["FZXS14"],
            text_align = TextAlign::Right,
            paint = new_paint(Color::from_rgb(129, 212, 250))
        ),
    )?;
    let mut text_img = Text2Image::from_text(
        text,
        70.0,
        text_params!(
            font_families = &["FZXS14"],
            text_align = TextAlign::Left,
            line_height = 1.25
        ),
    );
    text_img.layout(730.0);
    if text_img.height() > 450.0 {
        return Err(Error::TextOverLength(text.to_string()));
    }
    text_img.draw_on_canvas(canvas, (270, 320));
    canvas.reset_matrix();
    encode_png(surface.image_snapshot())
}

register_meme!(
    "nokia",
    nokia,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["无内鬼，继续交易"],
    keywords = &["诺基亚", "有内鬼"],
    date_created = local_date(2021, 12, 15),
    date_modified = local_date(2023, 2, 14),
);
