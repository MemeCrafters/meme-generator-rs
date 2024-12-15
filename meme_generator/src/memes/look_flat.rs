use skia_safe::{Color, FontStyle, IRect, Image};

use crate::{
    error::Error,
    manager::register_meme,
    meme::{DecodedImage, MemeOptions},
    utils::{
        canvas::CanvasExt, encoder::make_png_or_gif, image::ImageExt, local_date, new_surface,
        text::TextParams,
    },
};

#[derive(MemeOptions)]
struct Ratio {
    /// 图片“压扁”比例
    #[option(short, long, minimum = 1.0, maximum = 10.0, default = 2.0)]
    ratio: f32,
}

const DEFAULT_TEXT: &str = "可恶...被人看扁了";

fn look_flat(
    images: &mut Vec<DecodedImage>,
    texts: &Vec<String>,
    options: &Ratio,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };
    let ratio = options.ratio;

    let img_w = 500;
    let text_h = 80;
    let mut text_surface = new_surface((img_w, text_h));
    let canvas = text_surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(10, 0, img_w - 10, text_h),
        text,
        30.0,
        55.0,
        TextParams {
            font_style: FontStyle::bold(),
            ..Default::default()
        },
    )?;
    let text_image = text_surface.image_snapshot();

    let img_size = images[0].codec.dimensions();
    let img_w = 500;
    let img_h = img_size.height * img_w / img_size.width;
    let mut surface = new_surface((600, img_h + 230));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);

    let func = |images: &Vec<Image>| {
        let mut image = images[0].resize_width(img_w);
        image = image.resize_exact((img_w, (image.height() as f32 / ratio) as i32));
        let mut surface = new_surface((img_w, image.height() + text_h));
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        canvas.draw_image(&image, (0.0, 0.0), None);
        canvas.draw_image(&text_image, (0.0, image.height() as f32), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "look_flat",
    look_flat,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["看扁"],
    date_created = local_date(2022, 10, 6),
    date_modified = local_date(2023, 2, 14),
);
