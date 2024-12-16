use skia_safe::{Color, IRect, Image};

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

fn wechat_pay(
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

    let bg = load_image("wechat_pay/0.png")?;
    let mut surface = bg.to_surface();
    let canvas = surface.canvas();
    let qr_image = qrcode_image(message).resize_exact((530, 530));
    canvas.draw_image(&qr_image, (356, 439), None);
    canvas.draw_irect(
        IRect::from_xywh(528, 611, 186, 186),
        &new_paint(Color::WHITE),
    );
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(370, 1000, 872, 1100),
        name,
        50.0,
        80.0,
        None,
    )?;
    let logo = load_image("wechat_pay/logo.png")?;

    let func = |images: &Vec<Image>| {
        let mut surface = surface.clone();
        let canvas = surface.canvas();
        let image = images[0]
            .resize_fit((166, 166), Fit::Cover)
            .round_corner(8.0);
        canvas.draw_image(&image, (538, 621), None);
        canvas.draw_image(&logo, (649, 734), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "wechat_pay",
    wechat_pay,
    min_images = 1,
    max_images = 1,
    keywords = &["微信支付"],
    date_created = local_date(2024, 10, 30),
    date_modified = local_date(2024, 10, 30),
);
