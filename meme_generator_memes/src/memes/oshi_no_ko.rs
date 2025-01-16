use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    shortcut,
    text::Text2Image,
    text_params,
    tools::{load_image, local_date, new_stroke_paint, new_surface},
};

use crate::{register_meme, tags::MemeTags};

#[derive(MemeOptions)]
struct Name {
    /// 我推的名字
    #[option(short, long, default = "网友")]
    pub name: Option<String>,
}

fn oshi_no_ko(images: Vec<InputImage>, _: Vec<String>, options: Name) -> Result<Vec<u8>, Error> {
    let name = options.name.unwrap();

    let name_img = Text2Image::from_text(
        &name,
        150.0,
        text_params!(
            font_families = &["HiraginoMin"],
            stroke_paint = new_stroke_paint(Color::WHITE, 8.0)
        ),
    );
    if name_img.longest_line() > 800.0 {
        return Err(Error::TextOverLength(name.to_string()));
    }
    let text_h = name_img.height().ceil() as i32;
    let left = load_image("oshi_no_ko/text1.png")?.resize_height(text_h);
    let right = load_image("oshi_no_ko/text2.png")?.resize_height(text_h);
    let text_w = left.width() + name_img.longest_line() as i32 + right.width();
    let mut text_surface = new_surface((text_w, text_h));
    let canvas = text_surface.canvas();
    canvas.draw_image(&left, (0, 0), None);
    name_img.draw_on_canvas(&canvas, (left.width(), 0));
    canvas.draw_image(
        &right,
        (left.width() + name_img.longest_line() as i32, 0),
        None,
    );
    let text_img = text_surface.image_snapshot();
    let text_img = text_img.resize_width(663);

    let background = load_image("oshi_no_ko/background.png")?;
    let foreground = load_image("oshi_no_ko/foreground.png")?;

    let func = |images: Vec<Image>| {
        let mut surface = background.to_surface();
        let canvas = surface.canvas();
        let img = images[0].resize_fit((681, 692), Fit::Cover);
        canvas.draw_image(&img, (0, 0), None);
        canvas.draw_image(&text_img, (9, 102 - text_img.height() / 2), None);
        canvas.draw_image(&foreground, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "oshi_no_ko",
    oshi_no_ko,
    min_images = 1,
    max_images = 1,
    keywords = &["我推的网友"],
    shortcuts = &[shortcut!(
        r"我推的(?P<name>\S+)",
        options = &[("name", "{name}")],
        humanized = "我推的xx",
    )],
    tags = MemeTags::oshi_no_ko(),
    date_created = local_date(2023, 6, 1),
    date_modified = local_date(2023, 6, 23),
);
