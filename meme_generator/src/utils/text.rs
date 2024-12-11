use std::sync::{LazyLock, Mutex};

use skia_safe::{
    scalar,
    textlayout::{
        FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, TextAlign, TextStyle,
    },
    Canvas, Color, FontMgr, FontStyle, Paint, Point,
};

use crate::{config::MEME_CONFIG, utils::new_paint};

static FONT_MANAGER: LazyLock<Mutex<FontManager>> =
    LazyLock::new(|| Mutex::new(FontManager::init()));

struct FontManager {
    font_collection: FontCollection,
}

impl FontManager {
    pub fn init() -> Self {
        let mut font_collection = FontCollection::new();
        font_collection.set_default_font_manager(FontMgr::new(), None);
        Self {
            font_collection: font_collection,
        }
    }

    pub fn font_collection(&self) -> &FontCollection {
        &self.font_collection
    }
}

unsafe impl Send for FontManager {}

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
    pub fn from_text(text: impl Into<String>, font_size: scalar, text_params: &TextParams) -> Self {
        let text: String = text.into();
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
