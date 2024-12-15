use std::{
    collections::VecDeque,
    sync::{LazyLock, Mutex},
};

use skia_safe::{
    scalar,
    textlayout::{
        FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, TextAlign, TextDecoration,
        TextStyle, TypefaceFontProvider,
    },
    Canvas, Color, FontMgr, FontStyle, Paint, Point,
};

use crate::{
    config::{meme_home, MEME_CONFIG},
    utils::{color_from_hex_code, new_decoration, new_paint, new_stroke_paint},
};

static FONT_MANAGER: LazyLock<Mutex<FontManager>> =
    LazyLock::new(|| Mutex::new(FontManager::init()));

struct FontManager {
    font_collection: FontCollection,
}

impl FontManager {
    pub fn init() -> Self {
        let fonts_dir = meme_home().join("resources/fonts");
        let mut font_provider = TypefaceFontProvider::new();
        let font_mgr = FontMgr::new();
        if fonts_dir.exists() {
            let entries = fonts_dir.read_dir();
            if let Ok(entries) = entries {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_file() {
                            if let Some(ext) = path.extension() {
                                let ext = ext.to_str().unwrap();
                                if !["ttf", "ttc", "otf"].contains(&ext) {
                                    continue;
                                }
                                if let Ok(bytes) = std::fs::read(path.clone()) {
                                    if let Some(font) = font_mgr.new_from_data(&bytes, None) {
                                        font_provider.register_typeface(font, None);
                                    } else {
                                        eprintln!(
                                            "Failed to create typeface from font file: {path:?}",
                                        );
                                    }
                                } else {
                                    eprintln!("Failed to read font file: {path:?}");
                                }
                            }
                        }
                    }
                }
            } else {
                eprintln!("Failed to read fonts directory: {fonts_dir:?}");
            }
        }

        let mut font_collection = FontCollection::new();
        font_collection.set_default_font_manager(font_mgr, None);
        font_collection.set_asset_font_manager(FontMgr::from(font_provider));
        Self {
            font_collection: font_collection,
        }
    }

    pub fn font_collection(&self) -> &FontCollection {
        &self.font_collection
    }
}

unsafe impl Send for FontManager {}

#[derive(Debug, Clone)]
pub(crate) struct TextParams {
    pub font_style: FontStyle,
    pub font_families: Vec<String>,
    pub text_align: TextAlign,
    pub paint: Paint,
    pub stroke_paint: Option<Paint>,
}

impl Default for TextParams {
    fn default() -> Self {
        Self {
            font_style: FontStyle::default(),
            font_families: Vec::new(),
            text_align: TextAlign::Center,
            paint: new_paint(Color::BLACK),
            stroke_paint: None,
        }
    }
}

pub(crate) struct Text2Image {
    paragraph: Paragraph,
    stroke_paragraph: Option<Paragraph>,
}

impl Text2Image {
    pub fn from_text(
        text: impl Into<String>,
        font_size: scalar,
        text_params: impl Into<Option<TextParams>>,
    ) -> Self {
        let text: String = text.into();
        let text_params: TextParams = text_params.into().unwrap_or_default();
        let mut font_families = text_params.font_families.clone();
        font_families.append(&mut MEME_CONFIG.font.default_font_families.clone());

        let mut paragraph_style = ParagraphStyle::new();
        paragraph_style.set_text_align(text_params.text_align);

        let font_manager = FONT_MANAGER.lock().unwrap();
        let mut builder = ParagraphBuilder::new(&paragraph_style, font_manager.font_collection());
        let mut style = TextStyle::new();
        style.set_font_size(font_size);
        style.set_font_style(text_params.font_style);
        style.set_foreground_paint(&text_params.paint);
        style.set_font_families(&font_families);
        builder.push_style(&style);
        builder.add_text(text.clone());
        let mut paragraph = builder.build();
        paragraph.layout(scalar::INFINITY);

        let stroke_paragraph = match &text_params.stroke_paint {
            Some(stroke_paint) => {
                let mut stroke_builder =
                    ParagraphBuilder::new(&paragraph_style, font_manager.font_collection());
                let mut stroke_style = TextStyle::new();
                stroke_style.set_font_size(font_size);
                stroke_style.set_font_style(text_params.font_style);
                stroke_style.set_foreground_paint(&stroke_paint);
                stroke_style.set_font_families(&font_families);
                stroke_builder.push_style(&stroke_style);
                stroke_builder.add_text(text);
                let mut stroke_paragraph = stroke_builder.build();
                stroke_paragraph.layout(scalar::INFINITY);
                Some(stroke_paragraph)
            }
            None => None,
        };

        let mut text2image = Self {
            paragraph,
            stroke_paragraph,
        };
        text2image.layout(text2image.longest_line().ceil());
        text2image
    }

    pub fn from_bbcode_text(
        text: impl Into<String>,
        font_size: scalar,
        text_params: impl Into<Option<TextParams>>,
    ) -> Self {
        let text: String = text.into();
        let text_params: TextParams = text_params.into().unwrap_or_default();
        let mut font_families = text_params.font_families.clone();
        font_families.append(&mut MEME_CONFIG.font.default_font_families.clone());

        let mut paragraph_style = ParagraphStyle::new();
        paragraph_style.set_text_align(text_params.text_align);

        let font_manager = FONT_MANAGER.lock().unwrap();
        let mut builder = ParagraphBuilder::new(&paragraph_style, font_manager.font_collection());
        let mut style = TextStyle::new();
        style.set_font_size(font_size);
        style.set_font_style(text_params.font_style);
        style.set_foreground_paint(&text_params.paint);
        style.set_font_families(&font_families);
        builder.push_style(&style);

        let mut stroke_builder =
            ParagraphBuilder::new(&paragraph_style, font_manager.font_collection());
        let mut stroke_style = TextStyle::new();
        stroke_style.set_font_size(font_size);
        stroke_style.set_font_style(text_params.font_style);
        if let Some(stroke_paint) = &text_params.stroke_paint {
            stroke_style.set_foreground_paint(stroke_paint);
        }
        stroke_style.set_font_families(&font_families);
        stroke_builder.push_style(&stroke_style);

        let mut paint = text_params.paint;
        let mut stroke_paint = text_params
            .stroke_paint
            .unwrap_or(new_stroke_paint(Color::BLACK, 0.04 * font_size));

        let mut bold_stack = VecDeque::new();
        let mut italic_stack = VecDeque::new();
        let mut underline_stack = VecDeque::new();
        let mut strikethrough_stack = VecDeque::new();
        let mut color_stack = VecDeque::new();
        let mut stroke_stack = VecDeque::new();
        let mut has_stroke = false;

        let tokens = tokenize_bbcode(&text);
        for token in tokens {
            match token {
                BBCodeToken::OpenTag(tag) => match tag {
                    BBCodeTag::Bold => {
                        bold_stack.push_back(true);
                    }
                    BBCodeTag::Italic => {
                        italic_stack.push_back(true);
                    }
                    BBCodeTag::Underline => {
                        underline_stack.push_back(true);
                    }
                    BBCodeTag::Strikethrough => {
                        strikethrough_stack.push_back(true);
                    }
                    BBCodeTag::Color(color) => {
                        let color = color_from_hex_code(&color);
                        color_stack.push_back(color);
                    }
                    BBCodeTag::Stroke(color) => {
                        let color = color_from_hex_code(&color);
                        stroke_stack.push_back(color);
                        has_stroke = true;
                    }
                },
                BBCodeToken::CloseTag(tag) => match tag {
                    BBCodeTag::Bold => {
                        bold_stack.pop_back();
                    }
                    BBCodeTag::Italic => {
                        italic_stack.pop_back();
                    }
                    BBCodeTag::Underline => {
                        underline_stack.pop_back();
                    }
                    BBCodeTag::Strikethrough => {
                        strikethrough_stack.pop_back();
                    }
                    BBCodeTag::Color(_) => {
                        color_stack.pop_back();
                    }
                    BBCodeTag::Stroke(_) => {
                        stroke_stack.pop_back();
                    }
                },
                BBCodeToken::Text(text) => {
                    let bold = bold_stack.back().cloned().unwrap_or(false);
                    let italic = italic_stack.back().cloned().unwrap_or(false);
                    let underline = underline_stack.back().cloned().unwrap_or(false);
                    let strikethrough = strikethrough_stack.back().cloned().unwrap_or(false);
                    let color = color_stack.back().cloned().unwrap_or(paint.color());
                    let stroke_color = stroke_stack.back().cloned().unwrap_or(stroke_paint.color());

                    let font_style = if bold && italic {
                        FontStyle::bold_italic()
                    } else if bold {
                        FontStyle::bold()
                    } else if italic {
                        FontStyle::italic()
                    } else {
                        FontStyle::normal()
                    };
                    let text_decoration = if underline && strikethrough {
                        TextDecoration::UNDERLINE | TextDecoration::LINE_THROUGH
                    } else if underline {
                        TextDecoration::UNDERLINE
                    } else if strikethrough {
                        TextDecoration::LINE_THROUGH
                    } else {
                        TextDecoration::NO_DECORATION
                    };
                    let decoration = new_decoration(text_decoration, color);
                    style.set_font_style(font_style);
                    style.set_decoration(&decoration);
                    paint.set_color(color);
                    style.set_foreground_paint(&paint);

                    stroke_style.set_font_style(font_style);
                    stroke_style.set_decoration(&decoration);
                    stroke_paint.set_color(stroke_color);
                    stroke_style.set_foreground_paint(&stroke_paint);

                    builder.pop();
                    builder.push_style(&style);
                    builder.add_text(text.clone());
                    stroke_builder.pop();
                    stroke_builder.push_style(&stroke_style);
                    stroke_builder.add_text(text);
                }
            }
        }

        let mut paragraph = builder.build();
        paragraph.layout(scalar::INFINITY);

        let stroke_paragraph = if has_stroke {
            let mut stroke_paragraph = stroke_builder.build();
            stroke_paragraph.layout(scalar::INFINITY);
            Some(stroke_paragraph)
        } else {
            None
        };

        let mut text2image = Self {
            paragraph,
            stroke_paragraph,
        };
        text2image.layout(text2image.longest_line().ceil());
        text2image
    }

    pub fn longest_line(&self) -> scalar {
        self.paragraph.longest_line()
    }

    pub fn height(&self) -> scalar {
        self.paragraph.height()
    }

    pub fn layout(&mut self, width: scalar) {
        self.paragraph.layout(width);
        if let Some(stroke_paragraph) = &mut self.stroke_paragraph {
            stroke_paragraph.layout(width);
        }
    }

    pub fn draw_on_canvas(&self, canvas: &Canvas, origin: impl Into<Point>) {
        let origin: Point = origin.into();
        if let Some(stroke_paragraph) = &self.stroke_paragraph {
            stroke_paragraph.paint(canvas, origin);
        }
        self.paragraph.paint(canvas, origin);
    }
}

#[derive(Debug, Clone, PartialEq)]
enum BBCodeTag {
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Color(String),
    Stroke(String),
}

#[derive(Debug, Clone, PartialEq)]
enum BBCodeToken {
    Text(String),
    OpenTag(BBCodeTag),
    CloseTag(BBCodeTag),
}

fn tokenize_bbcode(input: &str) -> Vec<BBCodeToken> {
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < input.len() {
        if let Some(tag_start) = input[i..].find('[') {
            if tag_start > 0 {
                let text = input[i..i + tag_start].to_string();
                tokens.push(BBCodeToken::Text(text));
            }

            if let Some(tag_end) = input[i + tag_start..].find(']') {
                let tag = &input[i + tag_start + 1..i + tag_start + tag_end];
                let end_tag = tag.starts_with('/');
                let tag_name = if end_tag { &tag[1..] } else { tag };

                i += tag_start + tag_end + 1;

                if end_tag {
                    match tag_name {
                        "b" => tokens.push(BBCodeToken::CloseTag(BBCodeTag::Bold)),
                        "i" => tokens.push(BBCodeToken::CloseTag(BBCodeTag::Italic)),
                        "u" => tokens.push(BBCodeToken::CloseTag(BBCodeTag::Underline)),
                        "del" => tokens.push(BBCodeToken::CloseTag(BBCodeTag::Strikethrough)),
                        "color" => {
                            tokens.push(BBCodeToken::CloseTag(BBCodeTag::Color(String::new())))
                        }
                        "stroke" => {
                            tokens.push(BBCodeToken::CloseTag(BBCodeTag::Stroke(String::new())))
                        }
                        _ => {}
                    }
                } else {
                    if tag_name.starts_with("color=") {
                        let color_code = tag_name[6..].to_string();
                        tokens.push(BBCodeToken::OpenTag(BBCodeTag::Color(color_code)));
                    } else if tag_name.starts_with("stroke=") {
                        let stroke_code = tag_name[7..].to_string();
                        tokens.push(BBCodeToken::OpenTag(BBCodeTag::Stroke(stroke_code)));
                    } else {
                        match tag_name {
                            "b" => tokens.push(BBCodeToken::OpenTag(BBCodeTag::Bold)),
                            "i" => tokens.push(BBCodeToken::OpenTag(BBCodeTag::Italic)),
                            "u" => tokens.push(BBCodeToken::OpenTag(BBCodeTag::Underline)),
                            "del" => tokens.push(BBCodeToken::OpenTag(BBCodeTag::Strikethrough)),
                            _ => {}
                        }
                    }
                }
            } else {
                tokens.push(BBCodeToken::Text(input[i..].to_string()));
                break;
            }
        } else {
            tokens.push(BBCodeToken::Text(input[i..].to_string()));
            break;
        }
    }

    tokens
}
