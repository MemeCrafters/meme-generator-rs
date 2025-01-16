use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::register_meme;

#[derive(MemeOptions)]
struct Position {
    /// 奶茶的位置
    #[option(short, long, default="right", choices=["left", "right", "both"])]
    position: Option<String>,

    /// 左手
    #[option(long, long_aliases=["左手"], default = false)]
    left: Option<bool>,

    /// 右手
    #[option(long, long_aliases=["右手"], default = false)]
    right: Option<bool>,

    /// 双手
    #[option(long, long_aliases=["双手"], default = false)]
    both: Option<bool>,
}

fn bubble_tea(
    images: Vec<InputImage>,
    _: Vec<String>,
    options: Position,
) -> Result<Vec<u8>, Error> {
    let position = if options.left.unwrap() {
        "left"
    } else if options.right.unwrap() {
        "right"
    } else if options.both.unwrap() {
        "both"
    } else {
        options.position.as_deref().unwrap()
    };
    let left = position == "left" || position == "both";
    let right = position == "right" || position == "both";
    let bubble_tea = load_image("bubble_tea/0.png")?;

    let func = |images: Vec<Image>| {
        let frame = images[0].resize_fit((500, 500), Fit::Cover);
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        canvas.draw_image(&frame, (0, 0), None);
        if right {
            canvas.draw_image(&bubble_tea, (0, 0), None);
        }
        if left {
            canvas.draw_image(&bubble_tea.flip_horizontal(), (0, 0), None);
        }
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "bubble_tea",
    bubble_tea,
    min_images = 1,
    max_images = 1,
    keywords = &["奶茶"],
    date_created = local_date(2022, 8, 22),
    date_modified = local_date(2023, 3, 10),
);
