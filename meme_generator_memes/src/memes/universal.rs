use skia_safe::{textlayout::TextAlign, Color, FontStyle, Image, Point};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    encoder::make_png_or_gif,
    image::ImageExt,
    text::Text2Image,
    text_params,
    tools::{color_from_str, local_date, new_paint, new_stroke_paint, new_surface},
};

use crate::register_meme;

#[derive(MemeOptions)]
struct Options {
    /// 字体大小
    #[option(short, long, default = 50, minimum = 20, maximum = 100)]
    pub size: Option<i32>,

    /// 字体名
    #[option(short, long)]
    pub font: Option<String>,

    /// 文字颜色
    #[option(short, long)]
    pub color: Option<String>,

    /// 文字位置
    #[option(short, long, default = "bottom_outer", choices = ["top", "bottom", "center", "top_outer", "bottom_outer"])]
    pub position: Option<String>,

    /// 文字是否加粗
    #[option(short, long, default = false)]
    pub bold: Option<bool>,

    /// 文字是否倾斜
    #[option(short, long, default = false)]
    pub italic: Option<bool>,

    /// 文字旋转角度
    #[option(short, long, default = 0, minimum = -180, maximum = 180)]
    pub rotate: Option<i32>,

    /// 文字对齐
    #[option(short, long, default = "center", choices = ["left", "center", "right"])]
    pub align: Option<String>,

    /// 文字描边宽度
    #[option(long, minimum = 1, maximum = 10)]
    pub stroke_width: Option<i32>,

    /// 文字描边颜色
    #[option(long)]
    pub stroke_color: Option<String>,

    /// x方向偏移
    #[option(long, default = 0, minimum = -200, maximum = 200)]
    pub x_offset: Option<i32>,

    /// y方向偏移
    #[option(long, default = 0, minimum = -200, maximum = 200)]
    pub y_offset: Option<i32>,
}

fn universal(
    images: Vec<InputImage>,
    texts: Vec<String>,
    options: Options,
) -> Result<Vec<u8>, Error> {
    let text = &texts[0];

    let font_size = options.size.unwrap();
    let position = options.position.unwrap();
    let (default_color, default_stroke_color, default_stroke_width) = match position.as_str() {
        "top_outer" | "bottom_outer" => (Color::BLACK, None, None),
        "top" | "bottom" | "center" => (Color::WHITE, Some(Color::BLACK), Some(font_size / 10)),
        _ => unreachable!(),
    };
    let font_color = options.color.map_or(default_color, |c| color_from_str(&c));
    let stroke_color = options
        .stroke_color
        .map_or(default_stroke_color, |c| Some(color_from_str(&c)));
    let stroke_width = options
        .stroke_width
        .map_or(default_stroke_width, |w| Some(w));
    let font = options.font;
    let bold = options.bold.unwrap();
    let italic = options.italic.unwrap();
    let rotate = options.rotate.unwrap();
    let align = options.align.unwrap();
    let x_offset = options.x_offset.unwrap();
    let y_offset = options.y_offset.unwrap();

    let font_style = if bold && italic {
        FontStyle::bold_italic()
    } else if bold {
        FontStyle::bold()
    } else if italic {
        FontStyle::italic()
    } else {
        FontStyle::normal()
    };
    let text_align = match align.as_str() {
        "left" => TextAlign::Left,
        "center" => TextAlign::Center,
        "right" => TextAlign::Right,
        _ => unreachable!(),
    };
    let mut text_params = text_params!(
        font_style = font_style,
        paint = new_paint(font_color),
        text_align = text_align
    );
    if stroke_color.is_some() && stroke_width.is_some() {
        text_params.stroke_paint = Some(new_stroke_paint(
            stroke_color.unwrap(),
            stroke_width.unwrap() as f32,
        ));
    }
    if let Some(font) = font {
        text_params.font_families = vec![font];
    }

    let text2image = Text2Image::from_text(text, font_size as f32, text_params);
    let text_w = text2image.longest_line() as i32;
    let text_h = text2image.height() as i32;
    let padding = 10;

    let func = |imgs: Vec<Image>| {
        let img_w = 500;
        let img = imgs[0].resize_width(img_w);
        let img_h = img.height();

        let frame_w = img_w;
        let (frame_h, img_y, text_y) = match position.as_str() {
            "top" => (img_h, 0, 0),
            "bottom" => (img_h, 0, img_h - text_h - padding),
            "center" => (img_h, 0, (img_h - text_h) / 2),
            "top_outer" => (img_h + text_h + padding, text_h + padding, 0),
            "bottom_outer" => (img_h + text_h + padding, 0, img_h),
            _ => unreachable!(),
        };

        let mut surface = new_surface((frame_w, frame_h));
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        canvas.draw_image(&img, (0.0, img_y as f32), None);

        let center_x = frame_w / 2 + x_offset;
        let center_y = text_y + text_h / 2 + y_offset;
        canvas.rotate(
            rotate as f32,
            Some(Point::new(center_x as f32, center_y as f32)),
        );
        text2image.draw_on_canvas(&canvas, (center_x - text_w / 2, center_y - text_h / 2));
        canvas.reset_matrix();

        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "universal",
    universal,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 1,
    keywords = &["万能表情", "空白表情"],
    date_created = local_date(2022, 4, 20),
    date_modified = local_date(2025, 1, 13),
);
