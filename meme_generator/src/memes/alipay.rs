use skia_safe::{Color, IRect, Image, Rect};

use crate::{
    error::Error,
    manager::register_meme,
    meme::{DecodedImage, MemeOptions},
    utils::{
        canvas::CanvasExt,
        encoder::make_png_or_gif,
        image::{Fit, ImageExt},
        load_image, local_date, new_paint,
        tools::qrcode_image,
    },
};

const DEFAULT_MESSAGE: &str = "https://github.com/MemeCrafters/meme-generator-rs";

#[derive(MemeOptions)]
struct Message {
    /// 二维码内容
    #[option(short, long)]
    message: String,
}

fn alipay(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    options: &Message,
) -> Result<Vec<u8>, Error> {
    let message = if !options.message.is_empty() {
        &options.message
    } else {
        DEFAULT_MESSAGE
    };
    let name = images[0].name.clone();

    let bg = load_image("alipay/0.png")?;
    let mut surface = bg.to_surface();
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

    let func = |images: &Vec<Image>| {
        let mut surface = surface.clone();
        let canvas = surface.canvas();
        let image = images[0]
            .resize_fit((108, 108), Fit::Cover)
            .round_corner(8.0);
        canvas.draw_image(&image, (486, 881), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
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
