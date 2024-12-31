use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{MemeOptions, NamedImage},
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::register_meme;

#[derive(MemeOptions)]
struct Mode {
    /// 小丑在前/后
    #[option(short, long, default="front", choices = ["front", "behind"])]
    mode: Option<String>,

    /// 小丑在前
    #[option(long, short_aliases=['前'])]
    front: Option<bool>,

    /// 小丑在后
    #[option(long, short_aliases=['后'])]
    behind: Option<bool>,
}

fn clown_mask(images: Vec<NamedImage>, _: Vec<String>, options: Mode) -> Result<Vec<u8>, Error> {
    let make_front = |images: Vec<Image>| {
        let frame = load_image("clown_mask/0.png")?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        let image = images[0]
            .circle()
            .resize_exact((440, 440))
            .rotate_crop(-15.0);
        canvas.draw_image(&image, (16, 104), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    let make_behind = |images: Vec<Image>| {
        let frame1 = load_image("clown_mask/1.png")?;
        let frame2 = load_image("clown_mask/2.png")?;
        let mut surface = frame1.to_surface();
        let canvas = surface.canvas();
        let image = images[0]
            .circle()
            .perspective(&[(282, 0), (496, 154), (214, 546), (0, 392)])
            .rotate_crop(-6.0);
        canvas.draw_image(&image, (214, 100), None);
        canvas.draw_image(&frame2, (-85, 20), None);
        Ok(surface.image_snapshot())
    };

    let mode = if options.front.unwrap_or(false) {
        "front"
    } else if options.behind.unwrap_or(false) {
        "behind"
    } else {
        options.mode.as_deref().unwrap()
    };

    if mode == "front" {
        make_png_or_gif(images, make_front)
    } else {
        make_png_or_gif(images, make_behind)
    }
}

register_meme!(
    "clown_mask",
    clown_mask,
    min_images = 1,
    max_images = 1,
    keywords = &["小丑面具"],
    date_created = local_date(2024, 9, 20),
    date_modified = local_date(2024, 9, 20),
);
