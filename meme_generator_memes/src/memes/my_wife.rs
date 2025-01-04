use skia_safe::{Color, FontStyle, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{MemeOptions, NamedImage},
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_surface},
};

use crate::register_meme;

#[derive(MemeOptions)]
struct Options {
    /// 人称代词
    #[option(short, long, default = "我")]
    pub pronoun: Option<String>,

    /// 称呼
    #[option(short, long, default = "老婆")]
    pub name: Option<String>,
}

fn my_wife(images: Vec<NamedImage>, _: Vec<String>, options: Options) -> Result<Vec<u8>, Error> {
    let img = &images[0].image;
    let img_w = 400;
    let img_h = img.height() * img_w / img.width();
    let mut surface = new_surface((650, img_h + 500));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);

    let pronoun = options.pronoun.unwrap();
    let name = options.name.unwrap();
    let texts = [
        format!("如果你的{name}长这样"),
        format!("那么这就不是你的{name}\n这是{pronoun}的{name}"),
        format!("滚去找你\n自己的{name}去"),
    ];
    let rects = [
        IRect::from_ltrb(27, 12, 27 + 596, 12 + 79),
        IRect::from_ltrb(27, img_h + 120, 27 + 593, img_h + 120 + 135),
        IRect::from_ltrb(27, img_h + 295, 27 + 374, img_h + 295 + 135),
    ];
    for (i, text) in texts.iter().enumerate() {
        canvas.draw_text_area_auto_font_size(
            rects[i],
            text,
            30.0,
            70.0,
            text_params!(font_style = FontStyle::bold()),
        )?;
    }

    let point = load_image("my_wife/0.png")?.resize_width(200);
    canvas.draw_image(&point, (421, img_h + 270), None);

    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let img = images[0].resize_width(img_w);
        canvas.draw_image(&img, (325 - img_w / 2, 105), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "my_wife",
    my_wife,
    min_images = 1,
    max_images = 1,
    keywords = &["我老婆", "这是我老婆"],
    date_created = local_date(2022, 7, 29),
    date_modified = local_date(2024, 8, 12),
);
