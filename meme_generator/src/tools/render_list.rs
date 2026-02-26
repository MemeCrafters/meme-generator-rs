use std::{collections::HashMap, sync::LazyLock};

use serde::{Deserialize, Serialize};
use skia_safe::{Canvas, Color, Image, PaintJoin, Path, PathBuilder, Rect, textlayout::TextAlign};

use meme_generator_core::{error::Error, meme::Meme};
use meme_generator_utils::{
    encoder::encode_png,
    image::ImageExt,
    text::Text2Image,
    text_params,
    tools::{color_from_hex_code, new_paint, new_stroke_paint, new_surface},
};

use crate::memes::{MemeSortBy, get_memes_sorted};

fn draw_image_icon(color: Color) -> Image {
    let mut surface = new_surface((100, 100));
    let canvas = surface.canvas();

    let paint = new_paint(color);
    canvas.draw_circle((72.5, 26.5), 13.5, &paint);

    let mut paint = new_stroke_paint(color, 10.0);
    paint.set_stroke_join(PaintJoin::Miter);

    let mut builder = PathBuilder::new();
    builder.move_to((5.0, 5.0));
    builder.line_to((5.0, 95.0));
    builder.line_to((95.0, 95.0));
    builder.line_to((95.0, 5.0));
    builder.close();
    let path: Path = builder.into();
    canvas.draw_path(&path, &paint);

    let mut builder = PathBuilder::new();
    builder.move_to((5.0, 77.0));
    builder.line_to((37.5, 42.5));
    builder.line_to((61.5, 56.5));
    let path: Path = builder.into();
    canvas.draw_path(&path, &paint);

    let mut builder = PathBuilder::new();
    builder.move_to((34.0, 95.0));
    builder.line_to((69.5, 52.5));
    builder.line_to((95.0, 67.0));
    let path: Path = builder.into();
    canvas.draw_path(&path, &paint);

    surface.image_snapshot().resize_exact((30, 30))
}

fn draw_text_icon(color: Color) -> Image {
    let mut surface = new_surface((100, 100));
    let canvas = surface.canvas();

    let mut paint = new_stroke_paint(color, 10.0);
    paint.set_stroke_join(PaintJoin::Miter);

    let mut builder = PathBuilder::new();
    builder.move_to((5.0, 5.0));
    builder.line_to((5.0, 95.0));
    builder.line_to((95.0, 95.0));
    builder.line_to((95.0, 5.0));
    builder.close();
    let path: Path = builder.into();
    canvas.draw_path(&path, &paint);

    canvas.draw_line((20.0, 25.0), (80.0, 25.0), &paint);
    canvas.draw_line((50.0, 25.0), (50.0, 80.0), &paint);

    surface.image_snapshot().resize_exact((30, 30))
}

fn draw_hot_icon(color: Color) -> Image {
    let mut surface = new_surface((1024, 1024));
    let canvas = surface.canvas();
    let paint = new_paint(color);
    let path = Path::from_svg(
        "M702.08 558.72a469.12 469.12 0 0 0-50.56-210.56 776.64 776.64 0 0 0-105.6-186.56A778.24 778.24 0 0 0 467.2 86.4c-10.88-9.6-37.76-27.2-58.88-44.16S384 28.16 384 50.88c22.72 248-217.92 433.28-261.44 540.16-83.2 208.32 27.2 366.4 224 397.12 26.24 4.16 29.44-4.8 9.92-20.16a192 192 0 0 1-75.52-224c29.44-86.08 103.04-111.04 131.52-250.56 4.48-22.4 22.08-27.52 40.64-11.2a768 768 0 0 1 173.44 234.88c25.92 74.88 38.4 151.36-101.44 248.96-20.48 14.4 8.64 27.52 35.2 24.96C746.88 972.8 930.56 800 928 653.44c0-53.76-51.2-168-112.32-256-13.76-19.52-28.8-16.32-32 6.4-6.08 64-8.32 110.72-56 164.16-15.04 18.88-26.88 13.44-25.6-9.28z"
    ).unwrap();
    canvas.draw_path(&path, &paint);
    surface.image_snapshot().resize_exact((30, 30))
}

fn draw_new_icon(color: Color) -> Image {
    let mut surface = new_surface((1024, 1024));
    let canvas = surface.canvas();
    let paint = new_paint(color);
    let path = Path::from_svg(
        "M965.76 576l-74.56-78.08a107.2 107.2 0 0 1-19.84-105.6l40.96-99.84a40.96 40.96 0 0 0-39.68-60.8l-107.84-0.64a107.2 107.2 0 0 1-88.64-60.48l-41.6-99.52a40.96 40.96 0 0 0-71.04-14.72l-78.08 74.24a107.2 107.2 0 0 1-105.6 19.84L280 109.76a40.96 40.96 0 0 0-60.8 39.68L216.96 256a107.2 107.2 0 0 1-60.48 88.64l-99.52 41.6a40.96 40.96 0 0 0-14.72 71.04l74.56 78.08a107.52 107.52 0 0 1 19.84 105.6L96 741.76a40.96 40.96 0 0 0 39.68 60.8l107.84 2.56A107.2 107.2 0 0 1 331.84 864l41.6 99.52a40.96 40.96 0 0 0 71.04 14.72l78.08-74.56a107.2 107.2 0 0 1 105.6-19.84l99.84 40.96a40.96 40.96 0 0 0 60.8-39.68l2.56-107.84a107.2 107.2 0 0 1 60.48-88.64l99.52-41.6a40.96 40.96 0 0 0 14.4-71.04z m-309.44 128c-18.56 3.2-43.2 0-79.36-34.56-25.28-24.96-102.72-118.08-154.88-181.12l37.76 187.52a73.92 73.92 0 0 1-1.92 39.68 51.84 51.84 0 0 1-65.6 29.12A64 64 0 0 1 352 686.72l-50.56-288A59.52 59.52 0 0 1 311.68 352a64 64 0 0 1 43.52-21.76A55.36 55.36 0 0 1 416 352c26.24 29.76 52.16 59.84 77.76 89.92 32 36.8 61.12 73.92 91.52 110.72l-39.04-195.52a64 64 0 0 1 3.2-37.12 48.64 48.64 0 0 1 39.04-32 52.8 52.8 0 0 1 40.96 9.28 71.36 71.36 0 0 1 24.64 47.04L704 630.72A55.68 55.68 0 0 1 656.32 704z"
    ).unwrap();
    canvas.draw_path(&path, &paint);
    surface.image_snapshot().resize_exact((30, 30))
}

const COLOR_NORMAL: &str = "#444444";
const COLOR_DISABLED: &str = "#d3d3d3";
const BLOCK_COLOR_1: &str = "#f5f5f5";
const BLOCK_COLOR_2: &str = "#ffffff";
const BG_COLOR: &str = "#fdfcf8";
const FONTSIZE: f32 = 30.0;
const BLOCK_HEIGHT: f32 = 50.0;

static ICON_HOT: LazyLock<Image> = LazyLock::new(|| draw_hot_icon(color_from_hex_code("#d81e06")));
static ICON_NEW: LazyLock<Image> = LazyLock::new(|| draw_new_icon(color_from_hex_code("#ffa500")));

static ICON_IMAGE_NORMAL: LazyLock<Image> =
    LazyLock::new(|| draw_image_icon(color_from_hex_code(COLOR_NORMAL)));
static ICON_IMAGE_DISABLED: LazyLock<Image> =
    LazyLock::new(|| draw_image_icon(color_from_hex_code(COLOR_DISABLED)));

static ICON_TEXT_NORMAL: LazyLock<Image> =
    LazyLock::new(|| draw_text_icon(color_from_hex_code(COLOR_NORMAL)));
static ICON_TEXT_DISABLED: LazyLock<Image> =
    LazyLock::new(|| draw_text_icon(color_from_hex_code(COLOR_DISABLED)));

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MemeProperties {
    pub disabled: bool,
    pub hot: bool,
    pub new: bool,
}

impl Default for MemeProperties {
    fn default() -> Self {
        Self {
            disabled: false,
            hot: false,
            new: false,
        }
    }
}

enum MemeCategory {
    Image,
    Text,
}

fn format_string(template: &str, values: &HashMap<&str, String>) -> String {
    let mut result = template.to_string();
    for (key, value) in values {
        let placeholder = format!("{{{}}}", key);
        result = result.replace(&placeholder, value);
    }
    result
}

struct TextBlock {
    text2image: Text2Image,
    category: MemeCategory,
    add_category_icon: bool,
    block_color: Color,
    properties: MemeProperties,
}

impl TextBlock {
    fn new(
        text: &str,
        category: MemeCategory,
        add_category_icon: bool,
        block_color: Color,
        properties: MemeProperties,
    ) -> Self {
        let color = if properties.disabled {
            color_from_hex_code(COLOR_DISABLED)
        } else {
            color_from_hex_code(COLOR_NORMAL)
        };
        let text2image = Text2Image::from_text(
            text,
            FONTSIZE,
            text_params!(paint = new_paint(color), text_align = TextAlign::Left),
        );
        Self {
            text2image,
            category,
            add_category_icon,
            block_color,
            properties,
        }
    }

    fn width(&self) -> f32 {
        let mut width = self.text2image.longest_line();
        if self.add_category_icon {
            width += 50.0
        }
        width += 20.0;
        if self.properties.new {
            width += 35.0;
        }
        if self.properties.hot {
            width += 35.0;
        }
        width
    }

    fn draw_on_canvas(&self, canvas: &Canvas, pos: (f32, f32), block_width: f32) {
        canvas.draw_rect(
            Rect::from_xywh(pos.0, pos.1, block_width, BLOCK_HEIGHT),
            &new_paint(self.block_color),
        );
        let mut x = pos.0;
        if self.add_category_icon {
            let icon = match self.category {
                MemeCategory::Image => {
                    if self.properties.disabled {
                        &*ICON_IMAGE_DISABLED
                    } else {
                        &*ICON_IMAGE_NORMAL
                    }
                }
                MemeCategory::Text => {
                    if self.properties.disabled {
                        &*ICON_TEXT_DISABLED
                    } else {
                        &*ICON_TEXT_NORMAL
                    }
                }
            };
            canvas.draw_image(icon, (x + 10.0, pos.1 + 10.0), None);
            x += 50.0;
        }
        self.text2image.draw_on_canvas(
            canvas,
            (
                x + 5.0,
                pos.1 + (BLOCK_HEIGHT - self.text2image.height()) / 2.0,
            ),
        );
        x += self.text2image.longest_line() + 10.0;
        if self.properties.new {
            canvas.draw_image(&*ICON_NEW, (x + 5.0, pos.1 + 10.0), None);
            x += 35.0;
        }
        if self.properties.hot {
            canvas.draw_image(&*ICON_HOT, (x + 5.0, pos.1 + 10.0), None);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RenderMemeListParams {
    pub meme_properties: HashMap<String, MemeProperties>,
    pub exclude_memes: Vec<String>,
    pub sort_by: MemeSortBy,
    pub sort_reverse: bool,
    pub text_template: String,
    pub add_category_icon: bool,
}

impl Default for RenderMemeListParams {
    fn default() -> Self {
        Self {
            meme_properties: HashMap::new(),
            exclude_memes: Vec::new(),
            sort_by: MemeSortBy::KeywordsPinyin,
            sort_reverse: false,
            text_template: "{index}. {keywords}".to_string(),
            add_category_icon: true,
        }
    }
}

pub fn render_meme_list(params: RenderMemeListParams) -> Result<Vec<u8>, Error> {
    let memes = get_memes_sorted(params.sort_by, params.sort_reverse);

    let keywords = |meme: &Box<dyn Meme>| meme.info().keywords.join("/");
    let shortcuts = |meme: &Box<dyn Meme>| {
        meme.info()
            .shortcuts
            .iter()
            .map(|shortcut| shortcut.humanized.as_deref().unwrap_or(&shortcut.pattern))
            .collect::<Vec<_>>()
            .join("/")
    };
    let tags = |meme: &Box<dyn Meme>| meme.info().tags.into_iter().collect::<Vec<_>>().join("/");

    let meme_list = memes
        .iter()
        .filter(|meme| !params.exclude_memes.contains(&meme.key()))
        .map(|meme| {
            let key = meme.key();
            let properties = params
                .meme_properties
                .get(&key)
                .cloned()
                .unwrap_or_default();
            (meme, properties)
        })
        .collect::<Vec<_>>();

    let text_template = params.text_template;
    let add_category_icon = params.add_category_icon;

    let meme_text = |index: usize, meme: &Box<dyn Meme>| -> String {
        let mut vars = HashMap::new();
        vars.insert("index", (index + 1).to_string());
        vars.insert("key", meme.key());
        vars.insert("keywords", keywords(meme));
        vars.insert("shortcuts", shortcuts(meme));
        vars.insert("tags", tags(meme));
        format_string(&text_template, &vars)
    };

    let meme_num = meme_list.len();
    let cols = ((meme_num as f32 / 16.0).sqrt()).ceil() as usize;
    let rows = ((meme_num as f32 / cols as f32).ceil()) as usize;

    let mut text_blocks = Vec::new();
    let mut col_widths = Vec::new();

    for (col, col_meme_list) in meme_list.chunks(rows).enumerate() {
        let mut widths = Vec::new();
        for (row, (meme, properties)) in col_meme_list.iter().enumerate() {
            let text = meme_text(col * rows + row, meme);
            let block_color = if (row + col) % 2 == 0 {
                BLOCK_COLOR_1
            } else {
                BLOCK_COLOR_2
            };
            let category = if meme.info().params.max_images == 0 {
                MemeCategory::Text
            } else {
                MemeCategory::Image
            };
            let text_block = TextBlock::new(
                &text,
                category,
                add_category_icon,
                color_from_hex_code(block_color),
                properties.clone(),
            );
            widths.push(text_block.width());
            text_blocks.push(text_block);
        }
        let col_width = widths.into_iter().fold(0.0, f32::max);
        col_widths.push(col_width);
    }

    let margin = 30.0;
    let frame_w = col_widths.iter().sum::<f32>() + margin * 2.0;
    let frame_h = rows as f32 * BLOCK_HEIGHT + margin * 2.0;
    let mut surface = new_surface((frame_w as i32, frame_h as i32));
    let canvas = surface.canvas();
    canvas.clear(color_from_hex_code(BG_COLOR));
    let mut x = margin;
    for (col, col_meme_list) in meme_list.chunks(rows).enumerate() {
        let col_width = col_widths[col];
        let mut y = margin;
        for (row, _) in col_meme_list.iter().enumerate() {
            let text_block = &text_blocks[col * rows + row];
            text_block.draw_on_canvas(canvas, (x, y), col_width);
            y += BLOCK_HEIGHT;
        }
        x += col_width;
    }
    encode_png(surface.image_snapshot())
}
