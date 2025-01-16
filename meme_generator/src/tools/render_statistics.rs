use serde::{Deserialize, Serialize};
use skia_safe::{Color, Image, Path, PathEffect, Rect};

use meme_generator_utils::{
    encoder::encode_png,
    text::Text2Image,
    tools::{color_from_hex_code, new_paint, new_stroke_paint, new_surface},
};

use meme_generator_core::error::Error;

fn draw_line_chart(title: &str, data: Vec<(String, i32)>) -> Image {
    const AXIS_WIDTH: f32 = 600.0;
    const AXIS_HEIGHT: f32 = 400.0;
    const MARGIN: f32 = 50.0;
    const BG_COLOR: &str = "#eeeeee";
    const AXIS_COLOR: &str = "#bcbcbc";
    const LINE_COLOR: &str = "#348abd";

    let x_data = data.iter().map(|(label, _)| label).collect::<Vec<_>>();
    let y_data = data.iter().map(|(_, count)| count).collect::<Vec<_>>();
    let y_max = **y_data.iter().max().unwrap_or(&&1);

    let x_ticks = x_data.clone();
    let max_x_ticks = 12;
    let mut x_step = 1;
    let x_ticks = if x_ticks.len() > max_x_ticks {
        x_step = (x_ticks.len() as f32 / max_x_ticks as f32).ceil() as usize;
        x_ticks
            .iter()
            .enumerate()
            .filter_map(|(i, x)| if i % x_step == 0 { Some(*x) } else { None })
            .collect::<Vec<_>>()
    } else {
        x_ticks
    };

    let y_ticks = (0..=y_max).collect::<Vec<_>>();
    let max_y_ticks = 10;
    let mut y_step = 1;
    let y_ticks = if y_ticks.len() > max_y_ticks {
        y_step = (y_ticks.len() as f32 / max_y_ticks as f32).ceil() as usize;
        y_ticks
            .iter()
            .enumerate()
            .filter_map(|(i, y)| if i % y_step == 0 { Some(*y) } else { None })
            .collect::<Vec<_>>()
    } else {
        y_ticks
    };

    let chart_width = AXIS_WIDTH + 2.0 * MARGIN;
    let chart_height = AXIS_HEIGHT + 2.0 * MARGIN;
    let mut surface = new_surface((chart_width as i32, chart_height as i32));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);

    // 绘制背景
    let paint = new_paint(color_from_hex_code(BG_COLOR));
    canvas.draw_rect(
        Rect::from_xywh(MARGIN, MARGIN, AXIS_WIDTH, AXIS_HEIGHT),
        &paint,
    );

    // 绘制外边框
    let paint = new_stroke_paint(color_from_hex_code(AXIS_COLOR), 2.0);
    let mut path = Path::new();
    path.move_to((MARGIN, MARGIN));
    path.line_to((MARGIN, MARGIN + AXIS_HEIGHT));
    path.line_to((MARGIN + AXIS_WIDTH, MARGIN + AXIS_HEIGHT));
    path.line_to((MARGIN + AXIS_WIDTH, MARGIN));
    path.close();
    canvas.draw_path(&path, &paint);

    let x_grid_padding: f32 = 20.0;
    let y_grid_padding: f32 = 20.0;
    let x_sep = (AXIS_WIDTH - x_grid_padding * 2.0) / (x_data.len() - 1) as f32;
    let y_sep = (AXIS_HEIGHT - y_grid_padding * 2.0) / y_max as f32;
    let x_grid_sep = x_step as f32 * x_sep;
    let y_grid_sep = y_step as f32 * y_sep;

    // 绘制网格和刻度线
    let mut paint = new_stroke_paint(color_from_hex_code(AXIS_COLOR), 1.0);
    paint.set_path_effect(PathEffect::dash(&[5.0, 5.0], 0.0));
    let tick_length = 5.0;
    let tick_paint = new_stroke_paint(Color::BLACK, 1.0);
    let mut x = MARGIN + x_grid_padding;
    let y0 = MARGIN;
    let y1 = MARGIN + AXIS_HEIGHT;
    let y2 = MARGIN + AXIS_HEIGHT - tick_length;
    for _ in 0..x_ticks.len() {
        canvas.draw_line((x, y0), (x, y1), &paint);
        canvas.draw_line((x, y1), (x, y2), &tick_paint);
        x += x_grid_sep;
    }
    let mut y = chart_height - MARGIN - y_grid_padding;
    let x0 = MARGIN;
    let x1 = MARGIN + AXIS_WIDTH;
    let x2 = MARGIN + tick_length;
    for _ in 0..y_ticks.len() {
        canvas.draw_line((x0, y), (x1, y), &paint);
        canvas.draw_line((x0, y), (x2, y), &tick_paint);
        y -= y_grid_sep;
    }

    // 绘制数据点和连线
    let line_paint = new_stroke_paint(color_from_hex_code(LINE_COLOR), 2.0);
    let point_paint = new_paint(color_from_hex_code(LINE_COLOR));
    let mut path = Path::new();
    let mut x = MARGIN + x_grid_padding;
    for (i, y) in y_data.iter().enumerate() {
        let y = MARGIN + (AXIS_HEIGHT - (y_grid_padding + **y as f32 * y_sep));
        if i == 0 {
            path.move_to((x, y));
        } else {
            path.line_to((x, y));
        }
        canvas.draw_circle((x, y), 5.0, &point_paint);
        x += x_sep;
    }
    canvas.draw_path(&path, &line_paint);

    // 绘制 x 轴标签
    let mut x = MARGIN + x_grid_padding;
    let y = MARGIN + AXIS_HEIGHT + 2.0;
    let font_size = 16.0;
    for tick in x_ticks {
        let text2image = Text2Image::from_text(tick, font_size, None);
        text2image.draw_on_canvas(canvas, (x - text2image.longest_line() / 2.0, y));
        x += x_grid_sep;
    }

    // 绘制 y 轴标签
    let mut y = chart_height - MARGIN - y_grid_padding;
    let x = MARGIN - 8.0;
    for tick in y_ticks {
        let text2image = Text2Image::from_text(&tick.to_string(), font_size, None);
        text2image.draw_on_canvas(
            canvas,
            (x - text2image.longest_line(), y - text2image.height() / 2.0),
        );
        y -= y_grid_sep;
    }

    // 绘制标题
    let font_size = 20.0;
    let text2image = Text2Image::from_text(title, font_size, None);
    text2image.draw_on_canvas(
        canvas,
        (
            MARGIN + (AXIS_WIDTH - text2image.longest_line()) / 2.0,
            15.0,
        ),
    );

    surface.image_snapshot()
}

fn draw_bar_chart(title: &str, data: Vec<(String, i32)>) -> Image {
    const AXIS_WIDTH: f32 = 600.0;
    const MARGIN: f32 = 50.0;
    const BG_COLOR: &str = "#eeeeee";
    const AXIS_COLOR: &str = "#bcbcbc";
    const BAR_COLOR: &str = "#348abd";

    let mut data = data.clone();
    data.sort_by_key(|(_, count)| *count);

    let x_data = data.iter().map(|(_, count)| count).collect::<Vec<_>>();
    let y_data = data.iter().map(|(name, _)| name).collect::<Vec<_>>();
    let x_max = **x_data.iter().max().unwrap_or(&&1);

    let x_ticks = (0..=x_max).collect::<Vec<_>>();
    let max_x_ticks = 12;
    let mut x_step = 1;
    let x_ticks = if x_ticks.len() > max_x_ticks {
        x_step = (x_ticks.len() as f32 / max_x_ticks as f32).ceil() as usize;
        x_ticks
            .iter()
            .enumerate()
            .filter_map(|(i, x)| if i % x_step == 0 { Some(*x) } else { None })
            .collect::<Vec<_>>()
    } else {
        x_ticks
    };

    let y_ticks = y_data;

    let font_size = 16.0;
    let text2images = y_ticks
        .iter()
        .map(|y| Text2Image::from_text(*y, font_size, None))
        .collect::<Vec<_>>();
    let max_text_width = text2images
        .iter()
        .map(|t| t.longest_line())
        .fold(0.0, f32::max);

    let y_grid_padding: f32 = 30.0;
    let x_sep = (AXIS_WIDTH - y_grid_padding) / x_max as f32;
    let x_grid_sep = x_step as f32 * x_sep;
    let y_grid_sep: f32 = 40.0;
    let axis_height = (y_ticks.len() - 1) as f32 * y_grid_sep + 2.0 * y_grid_padding;

    let chart_width = AXIS_WIDTH + 2.0 * MARGIN + max_text_width;
    let chart_height = axis_height + 2.0 * MARGIN;
    let margin_left = MARGIN + max_text_width;
    let mut surface = new_surface((chart_width as i32, chart_height as i32));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);

    // 绘制背景
    let paint = new_paint(color_from_hex_code(BG_COLOR));
    canvas.draw_rect(
        Rect::from_xywh(margin_left, MARGIN, AXIS_WIDTH, axis_height),
        &paint,
    );

    // 绘制外边框
    let paint = new_stroke_paint(color_from_hex_code(AXIS_COLOR), 2.0);
    let mut path = Path::new();
    path.move_to((margin_left, MARGIN));
    path.line_to((margin_left, MARGIN + axis_height));
    path.line_to((margin_left + AXIS_WIDTH, MARGIN + axis_height));
    path.line_to((margin_left + AXIS_WIDTH, MARGIN));
    path.close();
    canvas.draw_path(&path, &paint);

    // 绘制网格和刻度线
    let mut paint = new_stroke_paint(color_from_hex_code(AXIS_COLOR), 1.0);
    paint.set_path_effect(PathEffect::dash(&[5.0, 5.0], 0.0));
    let tick_length = 5.0;
    let tick_paint = new_stroke_paint(Color::BLACK, 1.0);
    let mut x = margin_left + x_grid_sep;
    let y0 = MARGIN;
    let y1 = MARGIN + axis_height;
    let y2 = MARGIN + axis_height - tick_length;
    for _ in 1..x_ticks.len() {
        canvas.draw_line((x, y0), (x, y1), &paint);
        canvas.draw_line((x, y1), (x, y2), &tick_paint);
        x += x_grid_sep;
    }
    let mut y = MARGIN + y_grid_padding;
    let x0 = margin_left;
    let x1 = margin_left + AXIS_WIDTH;
    let x2 = margin_left + tick_length;
    for _ in 0..y_ticks.len() {
        canvas.draw_line((x0, y), (x1, y), &paint);
        canvas.draw_line((x0, y), (x2, y), &tick_paint);
        y += y_grid_sep;
    }

    // 绘制数据条
    let bar_height: f32 = 20.0;
    let mut y = chart_height - MARGIN - y_grid_padding;
    let paint = new_paint(color_from_hex_code(BAR_COLOR));
    for count in x_data {
        let bar_width = *count as f32 * x_sep;
        let rect = Rect::from_xywh(margin_left, y - bar_height / 2.0, bar_width, bar_height);
        canvas.draw_rect(rect, &paint);
        y -= y_grid_sep;
    }

    // 绘制 x 轴标签
    let mut x = margin_left;
    let y = MARGIN + axis_height + 2.0;
    let font_size = 16.0;
    for tick in x_ticks {
        let text2image = Text2Image::from_text(tick.to_string(), font_size, None);
        text2image.draw_on_canvas(canvas, (x - text2image.longest_line() / 2.0, y));
        x += x_grid_sep;
    }

    // 绘制 y 轴标签
    let mut y = MARGIN + y_grid_padding;
    let x = margin_left - 8.0;
    for tick in y_ticks {
        let text2image = Text2Image::from_text(&tick.to_string(), font_size, None);
        text2image.draw_on_canvas(
            canvas,
            (x - text2image.longest_line(), y - text2image.height() / 2.0),
        );
        y += y_grid_sep;
    }

    // 绘制标题
    let font_size = 20.0;
    let text2image = Text2Image::from_text(title, font_size, None);
    text2image.draw_on_canvas(
        canvas,
        (
            margin_left + (AXIS_WIDTH - text2image.longest_line()) / 2.0,
            15.0,
        ),
    );

    surface.image_snapshot()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemeStatisticsType {
    MemeCount,
    TimeCount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderMemeStatisticsParams {
    pub title: String,
    pub statistics_type: MemeStatisticsType,
    pub data: Vec<(String, i32)>,
}

pub fn render_meme_statistics(params: RenderMemeStatisticsParams) -> Result<Vec<u8>, Error> {
    let image = match params.statistics_type {
        MemeStatisticsType::MemeCount => draw_bar_chart(&params.title, params.data),
        MemeStatisticsType::TimeCount => draw_line_chart(&params.title, params.data),
    };
    encode_png(image)
}
