use skia_safe::{Color, Image};

use crate::{
    error::Error,
    manager::register_meme,
    meme::{DecodedImage, MemeOptions},
    utils::{
        encoder::make_png_or_gif,
        image::{Fit, ImageExt},
        load_image, local_date, new_surface,
    },
};

#[derive(MemeOptions)]
struct Position {
    /// 奶茶的位置
    #[option(short, long, default="right", choices=["left", "right", "both"])]
    position: String,

    /// 左手
    #[option(long)]
    left: bool,

    /// 右手
    #[option(long)]
    right: bool,

    /// 双手
    #[option(long)]
    both: bool,
}

fn bubble_tea(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    options: &Position,
) -> Result<Vec<u8>, Error> {
    let position = if options.left {
        "left"
    } else if options.right {
        "right"
    } else if options.both {
        "both"
    } else {
        options.position.as_str()
    };
    let left = position == "left" || position == "both";
    let right = position == "right" || position == "both";
    let bubble_tea = load_image("bubble_tea/0.png")?;

    let func = |images: &Vec<Image>| {
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
