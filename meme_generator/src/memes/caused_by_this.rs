use skia_safe::{Color, IRect, Image};

use crate::{
    error::Error,
    manager::register_meme,
    meme::{shortcut, DecodedImage},
    utils::{
        canvas::CanvasExt,
        encoder::make_png_or_gif,
        image::{Fit, ImageExt},
        load_image, local_date, new_surface,
        options::NoOptions,
        text::Text2Image,
    },
};

const DEFAULT_TEXT: &str = "心脏病 高血压 心律不齐 心肌梗塞 失眠 脱发 呼吸困难 胸闷气短 缺氧 躁郁 焦虑 脑供血不足 心慌心悸 心脑血管炸裂";

fn caused_by_this(
    images: &mut Vec<DecodedImage>,
    texts: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };

    let frame = load_image("caused_by_this/0.png")?;
    let text_img1 = Text2Image::from_text("你的", 55.0, None);
    let text_img2 = Text2Image::from_text("主要都是由这个引起的", 55.0, None);
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    text_img1.draw_on_canvas(&canvas, (10.0, 887.0 - text_img1.height() / 2.0));
    text_img2.draw_on_canvas(
        &canvas,
        (
            frame.width() as f32 - text_img2.longest_line() - 10.0,
            887.0 - text_img2.height() / 2.0,
        ),
    );
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(
            text_img1.longest_line() as i32 + 20,
            760,
            frame.width() - text_img2.longest_line() as i32 - 20,
            1000,
        ),
        text,
        10.0,
        60.0,
        None,
    )?;
    let frame = surface.image_snapshot();

    let func = |images: &Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].resize_fit((550, 360), Fit::Cover);
        canvas.draw_image(&image, (122, 9), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "caused_by_this",
    caused_by_this,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["这个引起的"],
    shortcuts = &[shortcut!(
        r"你的(?P<text>.+?)(?:主要)?都?是由?这个引起的",
        humanized = "你的xx主要都是由这个引起的",
        texts = &["{text}"],
    )],
    date_created = local_date(2024, 11, 18),
    date_modified = local_date(2024, 11, 22),
);
