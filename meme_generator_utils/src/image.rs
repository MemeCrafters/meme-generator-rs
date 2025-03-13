use skia_safe::{
    BlendMode, ClipOp, Color4f, ColorFilter, ColorMatrix, IRect, ISize, Image, ImageFilter, Matrix,
    Paint, Path, Point, RRect, Rect, SamplingOptions, Surface, canvas::SrcRectConstraint,
    color_filters, image_filters,
};

use crate::tools::{default_sampling_options, new_surface};

#[derive(Debug, Clone, PartialEq)]
pub enum Fit {
    /// 使图像完全包含在指定的大小内
    Contain,

    /// 使图像完全覆盖指定的大小
    Cover,
}

pub trait ImageExt {
    fn to_surface(&self) -> Surface;

    fn with_background(&self, color: impl Into<Color4f>) -> Image;

    fn resize_exact(&self, size: impl Into<ISize>) -> Image;

    fn resize_exact_with_sampling_options(
        &self,
        size: impl Into<ISize>,
        sampling: impl Into<SamplingOptions>,
    ) -> Image;

    fn resize_fit(&self, size: impl Into<ISize>, fit: Fit) -> Image;

    fn resize_bound(&self, size: impl Into<ISize>, fit: Fit) -> Image;

    fn resize_width(&self, width: i32) -> Image;

    fn resize_height(&self, height: i32) -> Image;

    fn crop(&self, rect: impl AsRef<IRect>) -> Image;

    fn square(&self) -> Image;

    fn clip_path(&self, path: &Path, op: ClipOp) -> Image;

    fn circle(&self) -> Image;

    fn round_corner(&self, radius: f32) -> Image;

    fn clip_mask(&self, mask: &Image) -> Image;

    fn rotate(&self, degrees: f32) -> Image;

    fn rotate_crop(&self, degrees: f32) -> Image;

    fn flip_vertical(&self) -> Image;

    fn flip_horizontal(&self) -> Image;

    fn perspective(&self, points: &[impl Into<Point> + Copy; 4]) -> Image;

    fn color_filter(&self, filter: ColorFilter) -> Image;

    fn color_matrix(&self, matrix: ColorMatrix) -> Image;

    fn grayscale(&self) -> Image;

    fn colorize(&self, color: impl Into<Color4f>) -> Image;

    fn invert(&self) -> Image;

    fn transparency(&self, factor: f32) -> Image;

    fn brightness(&self, factor: f32) -> Image;

    fn image_filter(&self, filter: ImageFilter) -> Image;

    fn gaussian_blur(&self, sigma: f32) -> Image;
}

impl ImageExt for Image {
    fn to_surface(&self) -> Surface {
        let mut surface = new_surface(self.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(self, (0, 0), None);
        surface
    }

    fn with_background(&self, color: impl Into<Color4f>) -> Image {
        let color = color.into();
        let mut surface = new_surface(self.dimensions());
        let canvas = surface.canvas();
        canvas.clear(color);
        canvas.draw_image(self, (0, 0), None);
        surface.image_snapshot()
    }

    fn resize_exact(&self, size: impl Into<ISize>) -> Image {
        self.resize_exact_with_sampling_options(size, default_sampling_options())
    }

    fn resize_exact_with_sampling_options(
        &self,
        size: impl Into<ISize>,
        sampling: impl Into<SamplingOptions>,
    ) -> Image {
        let size = size.into();
        let sampling = sampling.into();
        let mut surface = new_surface(size);
        let canvas = surface.canvas();
        let paint = Paint::default();
        canvas.draw_image_rect_with_sampling_options(
            self,
            Some((&Rect::from_irect(self.bounds()), SrcRectConstraint::Fast)),
            Rect::from_isize(size),
            sampling,
            &paint,
        );
        surface.image_snapshot()
    }

    fn resize_fit(&self, size: impl Into<ISize>, fit: Fit) -> Image {
        let size = size.into();
        let src = Rect::from_isize(self.dimensions());
        let dst = Rect::from_isize(size);

        let src = match fit {
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

        let dst = match fit {
            Fit::Contain => {
                let (width, height) = if dst.width() / dst.height() > src.width() / src.height() {
                    (src.width() * dst.height() / src.height(), dst.height())
                } else {
                    (dst.width(), src.height() * dst.width() / src.width())
                };
                Rect::from_xywh(
                    (dst.width() - width) / 2.0,
                    (dst.height() - height) / 2.0,
                    width,
                    height,
                )
            }
            Fit::Cover => dst.clone(),
        };

        let mut surface = new_surface(size);
        let canvas = surface.canvas();
        let paint = Paint::default();
        canvas.draw_image_rect_with_sampling_options(
            self,
            Some((&src, SrcRectConstraint::Fast)),
            dst,
            default_sampling_options(),
            &paint,
        );
        surface.image_snapshot()
    }

    fn resize_bound(&self, size: impl Into<ISize>, fit: Fit) -> Image {
        let size = size.into();
        let src = IRect::from_size(self.dimensions());
        let dst = IRect::from_size(size);

        let (width, height) = match fit {
            Fit::Contain => {
                if dst.width() / dst.height() > src.width() / src.height() {
                    (dst.width(), src.height() * dst.width() / src.width())
                } else {
                    (src.width() * dst.height() / src.height(), dst.height())
                }
            }
            Fit::Cover => {
                if dst.width() / dst.height() > src.width() / src.height() {
                    (src.width() * dst.height() / src.height(), dst.height())
                } else {
                    (dst.width(), src.height() * dst.width() / src.width())
                }
            }
        };
        self.resize_exact((width, height))
    }

    fn resize_width(&self, width: i32) -> Image {
        let height = ((self.height() as f32) * (width as f32) / (self.width() as f32)) as i32;
        self.resize_exact((width, height))
    }

    fn resize_height(&self, height: i32) -> Image {
        let width = ((self.width() as f32) * (height as f32) / (self.height() as f32)) as i32;
        self.resize_exact((width, height))
    }

    fn crop(&self, rect: impl AsRef<IRect>) -> Image {
        let rect = rect.as_ref();
        let mut surface = new_surface(rect.size());
        let canvas = surface.canvas();
        canvas.draw_image(self, (-rect.left() as f32, -rect.top() as f32), None);
        surface.image_snapshot()
    }

    fn square(&self) -> Image {
        let size = self.width().min(self.height());
        self.crop(&IRect::from_xywh(
            ((self.width() - size) as f32 / 2.0).round() as i32,
            ((self.height() - size) as f32 / 2.0).round() as i32,
            size,
            size,
        ))
    }

    fn clip_path(&self, path: &Path, op: ClipOp) -> Image {
        let mut surface = new_surface(self.dimensions());
        let canvas = surface.canvas();
        canvas.clip_path(path, op, true);
        canvas.draw_image(self, (0, 0), None);
        surface.image_snapshot()
    }

    fn circle(&self) -> Image {
        let image = self.square();
        let radius = image.width() as f32 / 2.0;
        let path = Path::circle((radius, radius), radius, None);
        image.clip_path(&path, ClipOp::Intersect)
    }

    fn round_corner(&self, radius: f32) -> Image {
        let path = Path::rrect(
            &RRect::new_rect_xy(
                Rect::from_wh(self.width() as f32, self.height() as f32),
                radius,
                radius,
            ),
            None,
        );
        self.clip_path(&path, ClipOp::Intersect)
    }

    fn clip_mask(&self, mask: &Image) -> Image {
        let mut surface = new_surface(self.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(mask, (0, 0), None);
        let mut paint = Paint::default();
        paint.set_blend_mode(BlendMode::SrcIn);
        canvas.draw_image(self, (0, 0), Some(&paint));
        surface.image_snapshot()
    }

    fn rotate(&self, degrees: f32) -> Image {
        let radians = degrees.to_radians();
        let width = self.width() as f32;
        let height = self.height() as f32;
        let abs_sin = radians.sin().abs();
        let abs_cos = radians.cos().abs();
        let rotated_width = width * abs_cos + height * abs_sin;
        let rotated_height = width * abs_sin + height * abs_cos;

        let mut surface = new_surface((rotated_width as i32, rotated_height as i32));
        let canvas = surface.canvas();
        canvas.translate((rotated_width / 2.0, rotated_height / 2.0));
        canvas.rotate(degrees, None);
        canvas.translate((-self.width() as f32 / 2.0, -self.height() as f32 / 2.0));
        canvas.draw_image_with_sampling_options(self, (0, 0), default_sampling_options(), None);
        surface.image_snapshot()
    }

    fn rotate_crop(&self, degrees: f32) -> Image {
        let mut surface = new_surface(self.dimensions());
        let canvas = surface.canvas();
        canvas.translate((self.width() as f32 / 2.0, self.height() as f32 / 2.0));
        canvas.rotate(degrees, None);
        canvas.translate((-self.width() as f32 / 2.0, -self.height() as f32 / 2.0));
        canvas.draw_image_with_sampling_options(self, (0, 0), default_sampling_options(), None);
        surface.image_snapshot()
    }

    fn flip_vertical(&self) -> Image {
        let mut surface = new_surface(self.dimensions());
        let canvas = surface.canvas();
        canvas.translate((0, self.height()));
        canvas.scale((1.0, -1.0));
        canvas.draw_image(self, (0, 0), None);
        surface.image_snapshot()
    }

    fn flip_horizontal(&self) -> Image {
        let mut surface = new_surface(self.dimensions());
        let canvas = surface.canvas();
        canvas.translate((self.width(), 0));
        canvas.scale((-1.0, 1.0));
        canvas.draw_image(self, (0, 0), None);
        surface.image_snapshot()
    }

    fn perspective(&self, points: &[impl Into<Point> + Copy; 4]) -> Image {
        let top_left: Point = points[0].into();
        let top_right: Point = points[1].into();
        let bottom_right: Point = points[2].into();
        let bottom_left: Point = points[3].into();

        let x1: f32 = top_left.x;
        let y1: f32 = top_left.y;
        let x2: f32 = top_right.x;
        let y2: f32 = top_right.y;
        let x3: f32 = bottom_right.x;
        let y3: f32 = bottom_right.y;
        let x4: f32 = bottom_left.x;
        let y4: f32 = bottom_left.y;

        let max_y = y1.max(y2).max(y3).max(y4);
        let min_y = y1.min(y2).min(y3).min(y4);
        let max_x = x1.max(x2).max(x3).max(x4);
        let min_x = x1.min(x2).min(x3).min(x4);
        let w = max_x - min_x;
        let h = max_y - min_y;

        let mut surface = new_surface((w as i32, h as i32));
        let canvas = surface.canvas();

        let matrix = Matrix::from_poly_to_poly(
            &[
                Point::new(0.0, 0.0),
                Point::new(self.width() as f32, 0.0),
                Point::new(self.width() as f32, self.height() as f32),
                Point::new(0.0, self.height() as f32),
            ],
            &[top_left, top_right, bottom_right, bottom_left],
        )
        .unwrap();

        canvas.concat(&matrix);
        canvas.draw_image_with_sampling_options(self, (0, 0), default_sampling_options(), None);
        surface.image_snapshot()
    }

    fn color_filter(&self, filter: ColorFilter) -> Image {
        let mut surface = new_surface(self.dimensions());
        let canvas = surface.canvas();
        let mut paint = Paint::default();
        paint.set_color_filter(filter);
        canvas.draw_image(self, (0, 0), Some(&paint));
        surface.image_snapshot()
    }

    fn color_matrix(&self, matrix: ColorMatrix) -> Image {
        self.color_filter(color_filters::matrix(&matrix, None))
    }

    fn grayscale(&self) -> Image {
        self.color_matrix(ColorMatrix::new(
            0.2126, 0.7152, 0.0722, 0.0, 0.0, //
            0.2126, 0.7152, 0.0722, 0.0, 0.0, //
            0.2126, 0.7152, 0.0722, 0.0, 0.0, //
            0.0, 0.0, 0.0, 1.0, 0.0,
        ))
    }

    fn colorize(&self, color: impl Into<Color4f>) -> Image {
        let color: Color4f = color.into();
        let color = color.to_color();
        self.color_filter(color_filters::blend(color, BlendMode::Color).unwrap())
    }

    fn invert(&self) -> Image {
        self.color_matrix(ColorMatrix::new(
            -1.0, 0.0, 0.0, 0.0, 1.0, //
            0.0, -1.0, 0.0, 0.0, 1.0, //
            0.0, 0.0, -1.0, 0.0, 1.0, //
            0.0, 0.0, 0.0, 1.0, 0.0,
        ))
    }

    fn transparency(&self, factor: f32) -> Image {
        self.color_matrix(ColorMatrix::new(
            1.0, 0.0, 0.0, 0.0, 0.0, //
            0.0, 1.0, 0.0, 0.0, 0.0, //
            0.0, 0.0, 1.0, 0.0, 0.0, //
            0.0, 0.0, 0.0, factor, 0.0,
        ))
    }

    fn brightness(&self, factor: f32) -> Image {
        self.color_matrix(ColorMatrix::new(
            factor, 0.0, 0.0, 0.0, 0.0, //
            0.0, factor, 0.0, 0.0, 0.0, //
            0.0, 0.0, factor, 0.0, 0.0, //
            0.0, 0.0, 0.0, 1.0, 0.0,
        ))
    }

    fn image_filter(&self, filter: ImageFilter) -> Image {
        let mut surface = new_surface(self.dimensions());
        let canvas = surface.canvas();
        let mut paint = Paint::default();
        paint.set_image_filter(filter);
        canvas.draw_image(self, (0, 0), Some(&paint));
        surface.image_snapshot()
    }

    fn gaussian_blur(&self, sigma: f32) -> Image {
        self.image_filter(image_filters::blur((sigma, sigma), None, None, None).unwrap())
    }
}
