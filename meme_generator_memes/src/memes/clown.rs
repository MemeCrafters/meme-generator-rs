use skia_safe::{Color, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{MemeOptions, NamedImage},
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::register_meme;

#[derive(MemeOptions)]
struct Person {
    /// 是否使用爷爷头轮廓
    #[option(short, long, short_aliases = ['爷'])]
    person: Option<bool>,
}

fn clown(images: Vec<NamedImage>, _: Vec<String>, options: Person) -> Result<Vec<u8>, Error> {
    let (frame_path, size, angle, left_center_x, center_y) = if options.person.unwrap_or(false) {
        ("clown/person.png", (434, 467), 26.0, 174, 378)
    } else {
        ("clown/circle.png", (554, 442), 26.0, 153, 341)
    };

    let func = |images: Vec<Image>| {
        let frame = load_image(frame_path)?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].resize_fit(size, Fit::Cover);

        let left_part = image
            .crop(IRect::from_ltrb(0, 0, image.width() / 2, image.height()))
            .rotate(-angle);
        let right_part = image
            .crop(IRect::from_ltrb(
                image.width() / 2,
                0,
                image.width(),
                image.height(),
            ))
            .rotate(angle);

        let left_top_x = left_center_x - left_part.width() / 2;
        let top_y = center_y - left_part.height() / 2;
        let right_top_x = frame.width() - left_top_x - right_part.width();

        canvas.draw_image(&left_part, (left_top_x, top_y), None);
        canvas.draw_image(&right_part, (right_top_x, top_y), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "clown",
    clown,
    min_images = 1,
    max_images = 1,
    keywords = &["小丑"],
    date_created = local_date(2023, 10, 14),
    date_modified = local_date(2023, 10, 14),
);
