use skia_safe::{Color, IRect, Rect};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    canvas::CanvasExt,
    encoder::encode_png,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_paint},
};

use crate::{register_meme, utils::qrcode_image};

const DEFAULT_MESSAGE: &str = "https://github.com/MemeCrafters/meme-generator-rs";

#[derive(MemeOptions)]
struct Message {
    /// 二维码内容
    #[option(short, long)]
    message: Option<String>,
}

fn alipay(images: Vec<InputImage>, _: Vec<String>, options: Message) -> Result<Vec<u8>, Error> {
    let message = options.message.as_deref().unwrap_or(DEFAULT_MESSAGE);
    let name = &images[0].name;

    let frame = load_image("alipay/0.png")?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    let qr_image = qrcode_image(message).resize_exact((658, 658));
    canvas.draw_image(&qr_image, (211, 606), None);
    canvas.draw_round_rect(
        Rect::from_xywh(482.0, 877.0, 116.0, 116.0),
        12.0,
        12.0,
        &new_paint(Color::WHITE),
    );
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(230, 1290, 850, 1380),
        name,
        40.0,
        70.0,
        None,
    )?;

    let image = images[0]
        .image
        .resize_fit((108, 108), Fit::Cover)
        .round_corner(8.0);
    canvas.draw_image(&image, (486, 881), None);
    encode_png(surface.image_snapshot())
}

register_meme!(
    "alipay",
    alipay,
    min_images = 1,
    max_images = 1,
    keywords = &["支付宝支付"],
    date_created = local_date(2024, 10, 30),
    date_modified = local_date(2024, 10, 30),
);
