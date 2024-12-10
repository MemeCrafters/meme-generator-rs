use skia_safe::{scalar, Canvas, Point, Rect};

use crate::{
    error::Error,
    utils::text::{Text2Image, TextParams},
};

pub trait CanvasExt {
    fn draw_text(
        &self,
        origin: impl Into<Point>,
        text: impl Into<String>,
        font_size: scalar,
        text_params: &TextParams,
    );

    fn draw_text_area(
        &self,
        rect: impl Into<Rect>,
        text: impl Into<String>,
        font_size: scalar,
        text_params: &TextParams,
    ) -> Result<(), Error>;

    fn draw_text_area_auto_font_size(
        &self,
        rect: impl Into<Rect>,
        text: impl Into<String>,
        max_font_size: scalar,
        min_font_size: scalar,
        text_params: &TextParams,
    ) -> Result<(), Error>;
}

impl CanvasExt for Canvas {
    fn draw_text(
        &self,
        origin: impl Into<Point>,
        text: impl Into<String>,
        font_size: scalar,
        text_params: &TextParams,
    ) {
        let origin: Point = origin.into();
        let mut text2image = Text2Image::from_text(text, font_size, text_params);
        text2image.draw_on_canvas(self, origin, None);
    }

    fn draw_text_area(
        &self,
        rect: impl Into<Rect>,
        text: impl Into<String>,
        font_size: scalar,
        text_params: &TextParams,
    ) -> Result<(), Error> {
        let rect: Rect = rect.into();
        let text: String = text.into();
        let mut text2image = Text2Image::from_text(text.clone(), font_size, text_params);
        text2image.layout(rect.width());
        if text2image.height() > rect.height() {
            return Err(Error::TextOverLength(text));
        }
        let top = rect.top() + (rect.height() - text2image.height()) / 2.0;
        text2image.draw_on_canvas(self, (rect.left(), top), rect.width());
        Ok(())
    }

    fn draw_text_area_auto_font_size(
        &self,
        rect: impl Into<Rect>,
        text: impl Into<String>,
        max_font_size: scalar,
        min_font_size: scalar,
        text_params: &TextParams,
    ) -> Result<(), Error> {
        let rect: Rect = rect.into();
        let text: String = text.into();
        let mut font_size = max_font_size;
        while font_size >= min_font_size {
            let mut text2image = Text2Image::from_text(text.clone(), font_size, text_params);
            text2image.layout(rect.width());
            if text2image.height() <= rect.height() {
                let top = rect.top() + (rect.height() - text2image.height()) / 2.0;
                text2image.draw_on_canvas(self, (rect.left(), top), rect.width());
                return Ok(());
            }
            font_size -= 1.0;
        }
        Err(Error::TextOverLength(text))
    }
}
