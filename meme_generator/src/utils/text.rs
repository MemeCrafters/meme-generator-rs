use std::sync::{LazyLock, Mutex};

use skia_safe::{
    scalar,
    textlayout::{
        FontCollection, Paragraph, ParagraphBuilder, ParagraphStyle, TextAlign, TextStyle,
    },
    Canvas, Color, Color4f, FontMgr, FontStyle, Image, Paint, Point, Rect,
};

use crate::{config::MEME_CONFIG, utils::new_surface};

static FONT_MANAGER: LazyLock<Mutex<FontManager>> =
    LazyLock::new(|| Mutex::new(FontManager::init()));

pub struct FontManager {
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

struct ParagraphWithStroke {
    paragraph: Paragraph,
    stroke_paragraph: Option<Paragraph>,
}

pub struct Text2Image {
    paragraph: ParagraphWithStroke,
}

pub struct TextParams {
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
            text_align: TextAlign::Left,
            paint: Paint::new(Color4f::from(Color::BLACK), None),
            stroke_paint: None,
        }
    }
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

        Self {
            paragraph: ParagraphWithStroke {
                paragraph,
                stroke_paragraph,
            },
        }
    }

    pub fn longest_line(&self) -> scalar {
        self.paragraph.paragraph.longest_line()
    }

    pub fn height(&self) -> scalar {
        self.paragraph.paragraph.height()
    }

    pub fn layout(&mut self, width: scalar) {
        self.paragraph.paragraph.layout(width);
        if let Some(stroke_paragraph) = &mut self.paragraph.stroke_paragraph {
            stroke_paragraph.layout(width);
        }
    }

    pub fn to_image(
        &mut self,
        max_width: impl Into<Option<scalar>>,
        padding: impl Into<Option<Rect>>,
    ) -> Image {
        let max_width: scalar = max_width.into().unwrap_or(self.longest_line().ceil());
        self.layout(max_width);

        let padding: Rect = padding.into().unwrap_or(Rect::default());
        let image_width = (max_width + padding.left + padding.right).ceil() as i32;
        let image_height = (self.height() + padding.top + padding.bottom).ceil() as i32;

        let mut surface = new_surface((image_width, image_height));
        let canvas = surface.canvas();

        let x = padding.left;
        let y = padding.top;
        if let Some(stroke_paragraph) = &self.paragraph.stroke_paragraph {
            stroke_paragraph.paint(&canvas, (x, y));
        }
        self.paragraph.paragraph.paint(&canvas, (x, y));

        surface.image_snapshot()
    }

    pub fn draw_on_canvas(
        &mut self,
        canvas: &Canvas,
        origin: impl Into<Point>,
        max_width: impl Into<Option<scalar>>,
    ) {
        let max_width: scalar = max_width.into().unwrap_or(self.longest_line().ceil());
        self.layout(max_width);

        let origin: Point = origin.into();
        if let Some(stroke_paragraph) = &self.paragraph.stroke_paragraph {
            stroke_paragraph.paint(canvas, origin);
        }
        self.paragraph.paragraph.paint(canvas, origin);
    }
}