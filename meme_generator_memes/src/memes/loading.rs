use skia_safe::{textlayout::TextAlign, Color, Color4f, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    canvas::CanvasExt,
    encoder::make_png_or_gif,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn loading(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let img = &images[0].image;
    let img_w = 500;
    let img_big = img.resize_width(img_w);
    let img_big = img_big.gaussian_blur(3.0);
    let h1 = img_big.height();
    let img_w_small = 100;
    let h2 = ((h1 as f32 / img_w as f32 * img_w_small as f32).round() as i32).max(80);

    let frame_h = h1 + h2 + 10;
    let mut surface = new_surface((img_w, frame_h));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    canvas.draw_image(&img_big, (0, 0), None);
    canvas.draw_irect(
        img_big.bounds(),
        &new_paint(Color4f::new(0.0, 0.0, 0.0, 0.3)),
    );
    let icon = load_image("loading/icon.png")?;
    canvas.draw_image(&icon, (200, (h1 / 2) - 50), None);
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(210, h1 + 5, 480, h1 + h2 + 5),
        "不出来",
        40.0,
        60.0,
        text_params!(text_align = TextAlign::Left),
    )?;
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let img_small = images[0].resize_width(img_w_small);
        canvas.draw_image(
            &img_small,
            (100, h1 + 5 + (h2 - img_small.height()) / 2),
            None,
        );
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "loading",
    loading,
    min_images = 1,
    max_images = 1,
    keywords = &["加载中"],
    date_created = local_date(2021, 12, 29),
    date_modified = local_date(2023, 2, 14),
);
