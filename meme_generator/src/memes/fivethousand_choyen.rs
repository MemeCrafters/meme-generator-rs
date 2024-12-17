use skia_safe::{Color, Image, Matrix, Paint, PaintJoin, PaintStyle, Shader, TileMode};

use crate::{
    error::Error,
    manager::register_meme,
    meme::DecodedImage,
    utils::{
        color_from_hex_code,
        encoder::encode_png,
        local_date, new_paint, new_stroke_paint, new_surface,
        options::NoOptions,
        text::{text_params, Text2Image},
    },
};

fn fivethousand_choyen(
    _: &mut Vec<DecodedImage>,
    texts: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let fontsize = 200.0;
    let font_families = &["Noto Sans SC"];
    let text = texts[0].clone();
    let pos_x = 20;
    let pos_y = 0;
    let mut images: Vec<(Image, (i32, i32))> = Vec::new();

    fn transform(text2image: Text2Image) -> Image {
        let tilt = -0.45;
        let text_w = text2image.longest_line();
        let text_h = text2image.height();
        let dw = (text_h * tilt).abs();
        let paddind_x = 20.0;
        let text_w = text_w + dw + paddind_x * 2.0;

        let mut surface = new_surface((text_w as i32, text_h as i32));
        let canvas = surface.canvas();
        let matrix = Matrix::from_affine(&[1.0, 0.0, tilt, 1.0, dw, 0.0]);
        canvas.concat(&matrix);
        text2image.draw_on_canvas(canvas, (paddind_x, 0.0));
        surface.image_snapshot()
    }

    let add_color_text = |text: &str,
                          font_families: &[&str],
                          images: &mut Vec<(Image, (i32, i32))>,
                          stroke_width: f32,
                          color: &str,
                          pos: (i32, i32)| {
        let color = color_from_hex_code(color);
        let text2image = Text2Image::from_text(
            text,
            fontsize,
            text_params!(
                font_families = font_families,
                paint = new_paint(color),
                stroke_paint = new_stroke_paint(color, stroke_width),
            ),
        );
        images.push((transform(text2image), (pos.0, pos.1)));
    };

    let add_gradient_text = |text: &str,
                             font_families: &[&str],
                             images: &mut Vec<(Image, (i32, i32))>,
                             stroke_width: f32,
                             dir: (i32, i32, i32, i32),
                             color_stops: Vec<(f32, (u8, u8, u8))>,
                             pos: (i32, i32)| {
        let shader = Shader::linear_gradient(
            ((dir.0 as f32, dir.1 as f32), (dir.2 as f32, dir.3 as f32)),
            color_stops
                .iter()
                .map(|(_, color)| Color::from_rgb(color.0, color.1, color.2))
                .collect::<Vec<_>>()
                .as_slice(),
            color_stops
                .iter()
                .map(|(pos, _)| *pos)
                .collect::<Vec<_>>()
                .as_slice(),
            TileMode::default(),
            None,
            None,
        );
        let mut paint = Paint::default();
        paint.set_shader(shader);
        let mut stroke_paint = paint.clone();
        stroke_paint.set_stroke_width(stroke_width);
        stroke_paint.set_style(PaintStyle::Stroke);
        stroke_paint.set_stroke_join(PaintJoin::Round);
        let text2image = Text2Image::from_text(
            text,
            fontsize,
            text_params!(
                font_families = font_families,
                paint = paint,
                stroke_paint = stroke_paint
            ),
        );
        images.push((transform(text2image), (pos.0, pos.1)));
    };

    // 黑
    add_color_text(
        &text,
        font_families,
        &mut images,
        44.0,
        "#000000",
        (pos_x + 8, pos_y + 8),
    );
    // 银
    add_gradient_text(
        &text,
        font_families,
        &mut images,
        40.0,
        (0, 38, 0, 234),
        vec![
            (0.0, (0, 15, 36)),
            (0.1, (255, 255, 255)),
            (0.18, (55, 58, 59)),
            (0.25, (55, 58, 59)),
            (0.5, (200, 200, 200)),
            (0.75, (55, 58, 59)),
            (0.85, (25, 20, 31)),
            (0.91, (240, 240, 240)),
            (0.95, (166, 175, 194)),
            (1.0, (50, 50, 50)),
        ],
        (pos_x + 8, pos_y + 8),
    );
    // 黑
    add_color_text(
        &text,
        font_families,
        &mut images,
        32.0,
        "#000000",
        (pos_x, pos_y),
    );
    // 金
    add_gradient_text(
        &text,
        font_families,
        &mut images,
        20.0,
        (0, 40, 0, 200),
        vec![
            (0.0, (253, 241, 0)),
            (0.25, (245, 253, 187)),
            (0.4, (255, 255, 255)),
            (0.75, (253, 219, 9)),
            (0.9, (127, 53, 0)),
            (1.0, (243, 196, 11)),
        ],
        (pos_x, pos_y),
    );
    // 黑
    add_color_text(
        &text,
        font_families,
        &mut images,
        12.0,
        "#000000",
        (pos_x + 4, pos_y - 6),
    );
    // 白
    add_color_text(
        &text,
        font_families,
        &mut images,
        12.0,
        "#ffffff",
        (pos_x + 0, pos_y - 6),
    );
    // 红
    add_gradient_text(
        &text,
        font_families,
        &mut images,
        8.0,
        (0, 50, 0, 200),
        vec![
            (0.0, (255, 100, 0)),
            (0.5, (123, 0, 0)),
            (0.51, (240, 0, 0)),
            (1.0, (5, 0, 0)),
        ],
        (pos_x, pos_y - 6),
    );
    // 红
    add_gradient_text(
        &text,
        font_families,
        &mut images,
        0.0,
        (0, 50, 0, 200),
        vec![
            (0.0, (230, 0, 0)),
            (0.5, (123, 0, 0)),
            (0.51, (240, 0, 0)),
            (1.0, (5, 0, 0)),
        ],
        (pos_x, pos_y - 6),
    );

    let text = texts[1].clone();
    let font_families = &["Noto Serif SC"];
    let pos_x = 280;
    let pos_y = 260;
    // 黑
    add_color_text(
        &text,
        font_families,
        &mut images,
        44.0,
        "#000000",
        (pos_x + 10, pos_y + 4),
    );
    // 银
    add_gradient_text(
        &text,
        font_families,
        &mut images,
        38.0,
        (0, 60, 0, 246),
        vec![
            (0.0, (0, 15, 36)),
            (0.25, (250, 250, 250)),
            (0.5, (150, 150, 150)),
            (0.75, (55, 58, 59)),
            (0.85, (25, 20, 31)),
            (0.91, (240, 240, 240)),
            (0.95, (166, 175, 194)),
            (1.0, (50, 50, 50)),
        ],
        (pos_x + 10, pos_y + 4),
    );
    // 黑
    add_color_text(
        &text,
        font_families,
        &mut images,
        34.0,
        "#10193A",
        (pos_x, pos_y),
    );
    // 白
    add_color_text(
        &text,
        font_families,
        &mut images,
        16.0,
        "#D0D0D0",
        (pos_x, pos_y),
    );
    // 绀
    add_gradient_text(
        &text,
        font_families,
        &mut images,
        14.0,
        (0, 60, 0, 220),
        vec![
            (0.0, (16, 25, 58)),
            (0.03, (255, 255, 255)),
            (0.08, (16, 25, 58)),
            (0.2, (16, 25, 58)),
            (1.0, (16, 25, 58)),
        ],
        (pos_x, pos_y),
    );
    // 银
    add_gradient_text(
        &text,
        font_families,
        &mut images,
        0.0,
        (0, 60, 0, 220),
        vec![
            (0.0, (245, 246, 248)),
            (0.15, (255, 255, 255)),
            (0.35, (195, 213, 220)),
            (0.5, (160, 190, 201)),
            (0.51, (160, 190, 201)),
            (0.52, (196, 215, 222)),
            (1.0, (255, 255, 255)),
        ],
        (pos_x, pos_y - 6),
    );

    let frame_h = 580;
    let frame_w = images
        .iter()
        .map(|(img, pos)| img.width() + pos.0)
        .max()
        .unwrap();
    let mut surface = new_surface((frame_w, frame_h));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    for (img, pos) in images {
        canvas.draw_image(img, (pos.0, pos.1), None);
    }
    encode_png(&surface.image_snapshot())
}

register_meme!(
    "5000choyen",
    fivethousand_choyen,
    min_texts = 2,
    max_texts = 2,
    default_texts = &["我去", "洛天依"],
    keywords = &["5000兆"],
    date_created = local_date(2022, 10, 29),
    date_modified = local_date(2024, 11, 2),
);
