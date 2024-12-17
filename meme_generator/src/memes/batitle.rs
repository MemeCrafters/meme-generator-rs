use skia_safe::{Color, Matrix};

use crate::{
    error::Error,
    manager::register_meme,
    meme::DecodedImage,
    tags::MemeTags,
    utils::{
        encoder::encode_png,
        load_image, local_date, new_stroke_paint, new_surface,
        options::NoOptions,
        text::{text_params, Text2Image},
    },
};

fn batitle(
    _: &mut Vec<DecodedImage>,
    texts: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let fontsize = 168.0;
    let font_families = &["Ro GSan Serif Std", "Glow Sans SC"];
    let tilt = -0.4;
    let color_blue = "#128AFA";
    let color_gray = "#2B2B2B";

    let text2image = Text2Image::from_bbcode_text(
        format!(
            "[color={}]{}[/color][color={}][stroke=#ffffff]{}[/stroke][/color]",
            color_blue, texts[0], color_gray, texts[1],
        ),
        fontsize,
        text_params!(
            font_families = font_families,
            stroke_paint = new_stroke_paint(Color::WHITE, 20.0),
        ),
    );

    let text_w = text2image.longest_line();
    let text_h = text2image.height();
    let dw = (text_h * tilt).abs();
    let text_w = text_w + dw;

    let mut surface = new_surface((text_w as i32, text_h as i32));
    let canvas = surface.canvas();
    let matrix = Matrix::from_affine(&[1.0, 0.0, tilt, 1.0, dw, 0.0]);
    canvas.concat(&matrix);
    text2image.draw_on_canvas(canvas, (0, 0));
    let text_image = surface.image_snapshot();

    let padding_x = 50;
    let frame_w = text_image.width() + padding_x * 2;
    let frame_h = 450;
    let text_y = 120;
    let logo_y = 10;
    let left_x = Text2Image::from_text(
        texts[0].as_str(),
        fontsize,
        text_params!(font_families = font_families),
    )
    .longest_line() as i32;
    let logo_x = padding_x + left_x - 100;
    let halo = load_image("batitle/halo.png")?;
    let cross = load_image("batitle/cross.png")?;

    let mut surface = new_surface((frame_w, frame_h));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    canvas.draw_image(&halo, (logo_x, logo_y), None);
    canvas.draw_image(&text_image, (padding_x, text_y), None);
    canvas.draw_image(&cross, (logo_x, logo_y), None);
    encode_png(&surface.image_snapshot())
}

register_meme!(
    "batitle",
    batitle,
    min_texts = 2,
    max_texts = 2,
    default_texts = &["Blue", "Archive"],
    keywords = &["蔚蓝档案标题", "batitle"],
    tags = MemeTags::blue_archive(),
    date_created = local_date(2023, 10, 14),
    date_modified = local_date(2024, 11, 2),
);
