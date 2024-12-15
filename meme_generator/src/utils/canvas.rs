use skia_safe::{scalar, Canvas, Point, Rect};

use crate::{
    error::Error,
    utils::text::{Text2Image, TextParams},
};

#[allow(dead_code)]
pub(crate) trait CanvasExt {
    fn draw_text(
        &self,
        origin: impl Into<Point>,
        text: impl Into<String>,
        font_size: scalar,
        text_params: impl Into<Option<TextParams>>,
    );

    fn draw_text_area(
        &self,
        rect: impl Into<Rect>,
        text: impl Into<String>,
        font_size: scalar,
        text_params: impl Into<Option<TextParams>>,
    ) -> Result<(), Error>;

    fn draw_text_area_auto_font_size(
        &self,
        rect: impl Into<Rect>,
        text: impl Into<String>,
        min_font_size: scalar,
        max_font_size: scalar,
        text_params: impl Into<Option<TextParams>>,
    ) -> Result<(), Error>;

    fn draw_bbcode_text(
        &self,
        origin: impl Into<Point>,
        text: impl Into<String>,
        font_size: scalar,
        text_params: impl Into<Option<TextParams>>,
    );

    fn draw_bbcode_text_area(
        &self,
        rect: impl Into<Rect>,
        text: impl Into<String>,
        font_size: scalar,
        text_params: impl Into<Option<TextParams>>,
    ) -> Result<(), Error>;

    fn draw_bbcode_text_area_auto_font_size(
        &self,
        rect: impl Into<Rect>,
        text: impl Into<String>,
        min_font_size: scalar,
        max_font_size: scalar,
        text_params: impl Into<Option<TextParams>>,
    ) -> Result<(), Error>;
}

fn draw_text(
    canvas: &Canvas,
    origin: impl Into<Point>,
    text: impl Into<String>,
    font_size: scalar,
    text_params: impl Into<Option<TextParams>>,
    use_bbcode: bool,
) {
    let origin: Point = origin.into();
    let text2image = if use_bbcode {
        Text2Image::from_bbcode_text(text, font_size, text_params)
    } else {
        Text2Image::from_text(text, font_size, text_params)
    };
    text2image.draw_on_canvas(canvas, origin);
}

fn draw_text_area(
    canvas: &Canvas,
    rect: impl Into<Rect>,
    text: impl Into<String>,
    font_size: scalar,
    text_params: impl Into<Option<TextParams>>,
    use_bbcode: bool,
) -> Result<(), Error> {
    let rect: Rect = rect.into();
    let text: String = text.into();
    let mut text2image = if use_bbcode {
        Text2Image::from_bbcode_text(text.clone(), font_size, text_params)
    } else {
        Text2Image::from_text(text.clone(), font_size, text_params)
    };
    text2image.layout(rect.width());
    if text2image.height() > rect.height() {
        return Err(Error::TextOverLength(text));
    }
    let top = rect.top() + (rect.height() - text2image.height()) / 2.0;
    text2image.draw_on_canvas(canvas, (rect.left(), top));
    Ok(())
}

fn draw_text_area_auto_font_size(
    canvas: &Canvas,
    rect: impl Into<Rect>,
    text: impl Into<String>,
    min_font_size: scalar,
    max_font_size: scalar,
    text_params: impl Into<Option<TextParams>>,
    use_bbcode: bool,
) -> Result<(), Error> {
    let rect: Rect = rect.into();
    let text: String = text.into();
    let text_params: TextParams = text_params.into().unwrap_or_default();
    let mut font_size = max_font_size;
    while font_size >= min_font_size {
        let mut text2image = if use_bbcode {
            Text2Image::from_bbcode_text(text.clone(), font_size, text_params.clone())
        } else {
            Text2Image::from_text(text.clone(), font_size, text_params.clone())
        };
        text2image.layout(rect.width());
        if text2image.height() <= rect.height() {
            let top = rect.top() + (rect.height() - text2image.height()) / 2.0;
            text2image.draw_on_canvas(canvas, (rect.left(), top));
            return Ok(());
        }
        font_size -= 1.0;
    }
    Err(Error::TextOverLength(text))
}

impl CanvasExt for Canvas {
    fn draw_text(
        &self,
        origin: impl Into<Point>,
        text: impl Into<String>,
        font_size: scalar,
        text_params: impl Into<Option<TextParams>>,
    ) {
        draw_text(self, origin, text, font_size, text_params, false);
    }

    fn draw_text_area(
        &self,
        rect: impl Into<Rect>,
        text: impl Into<String>,
        font_size: scalar,
        text_params: impl Into<Option<TextParams>>,
    ) -> Result<(), Error> {
        draw_text_area(self, rect, text, font_size, text_params, false)
    }

    fn draw_text_area_auto_font_size(
        &self,
        rect: impl Into<Rect>,
        text: impl Into<String>,
        min_font_size: scalar,
        max_font_size: scalar,
        text_params: impl Into<Option<TextParams>>,
    ) -> Result<(), Error> {
        draw_text_area_auto_font_size(
            self,
            rect,
            text,
            min_font_size,
            max_font_size,
            text_params,
            false,
        )
    }

    fn draw_bbcode_text(
        &self,
        origin: impl Into<Point>,
        text: impl Into<String>,
        font_size: scalar,
        text_params: impl Into<Option<TextParams>>,
    ) {
        draw_text(self, origin, text, font_size, text_params, true);
    }

    fn draw_bbcode_text_area(
        &self,
        rect: impl Into<Rect>,
        text: impl Into<String>,
        font_size: scalar,
        text_params: impl Into<Option<TextParams>>,
    ) -> Result<(), Error> {
        draw_text_area(self, rect, text, font_size, text_params, true)
    }

    fn draw_bbcode_text_area_auto_font_size(
        &self,
        rect: impl Into<Rect>,
        text: impl Into<String>,
        min_font_size: scalar,
        max_font_size: scalar,
        text_params: impl Into<Option<TextParams>>,
    ) -> Result<(), Error> {
        draw_text_area_auto_font_size(
            self,
            rect,
            text,
            min_font_size,
            max_font_size,
            text_params,
            true,
        )
    }
}
