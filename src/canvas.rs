use skia_safe::canvas::SrcRectConstraint;
use skia_safe::{Canvas, FilterMode, Image, MipmapMode, Paint, Rect, SamplingOptions};

pub enum Fit {
    Fill,
    Contain,
    Cover,
}

pub trait CanvasExt {
    fn draw_image_rect_fit(&self, image: impl AsRef<Image>, dst: impl AsRef<Rect>, fit: Fit);

    fn draw_image_rect_fit_with_sampling_options(
        &self,
        image: impl AsRef<Image>,
        dst: impl AsRef<Rect>,
        fit: Fit,
        sampling: impl Into<SamplingOptions>,
    );
}

impl CanvasExt for Canvas {
    fn draw_image_rect_fit(&self, image: impl AsRef<Image>, dst: impl AsRef<Rect>, fit: Fit) {
        let sampling_options = SamplingOptions::new(FilterMode::Linear, MipmapMode::Linear);
        self.draw_image_rect_fit_with_sampling_options(image, dst, fit, sampling_options);
    }

    fn draw_image_rect_fit_with_sampling_options(
        &self,
        image: impl AsRef<Image>,
        dst: impl AsRef<Rect>,
        fit: Fit,
        sampling: impl Into<SamplingOptions>,
    ) {
        let image = image.as_ref();
        let dst = dst.as_ref();
        let src = Rect::from_wh(image.width() as f32, image.height() as f32);

        let src_rect = match fit {
            Fit::Fill => src.clone(),
            Fit::Contain => src.clone(),
            Fit::Cover => {
                let (width, height) = if dst.width() / dst.height() > src.width() / src.height() {
                    (src.width(), src.width() * dst.height() / dst.width())
                } else {
                    (src.height() * dst.width() / dst.height(), src.height())
                };
                Rect::from_xywh(
                    (src.width() - width) / 2.0,
                    (src.height() - height) / 2.0,
                    width,
                    height,
                )
            }
        };

        let dst_rect = match fit {
            Fit::Fill => dst.clone(),
            Fit::Contain => {
                let (width, height) = if dst.width() / dst.height() > src.width() / src.height() {
                    (src.width() * dst.height() / src.height(), dst.height())
                } else {
                    (dst.width(), src.height() * dst.width() / src.width())
                };
                Rect::from_xywh(
                    dst.left() + (dst.width() - width) / 2.0,
                    dst.top() + (dst.height() - height) / 2.0,
                    width * dst.width() / src.width(),
                    height * dst.height() / src.height(),
                )
            }
            Fit::Cover => dst.clone(),
        };

        let paint = Paint::default();
        self.draw_image_rect_with_sampling_options(
            image,
            Some((&src_rect, SrcRectConstraint::Fast)),
            dst_rect,
            sampling,
            &paint,
        );
    }
}
