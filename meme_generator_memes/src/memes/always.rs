use skia_safe::{Color, IRect, Image, textlayout::TextAlign};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    canvas::CanvasExt,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif, make_png_or_gif},
    image::ImageExt,
    shortcut, text_params,
    tools::{local_date, new_surface},
};

use crate::register_meme;

#[derive(MemeOptions)]
struct Mode {
    /// 生成模式
    #[option(long, default = "normal", choices = ["normal", "loop", "circle"])]
    mode: Option<String>,

    /// 套娃模式
    #[option(long, long_aliases = ["套娃"], default=false)]
    circle: Option<bool>,

    /// 循环模式
    #[option(long, long_aliases = ["循环"], default=false)]
    r#loop: Option<bool>,
}

fn always_normal(images: Vec<InputImage>) -> Result<Vec<u8>, Error> {
    let img = &images[0].image;
    let ratio = img.height() as f32 / img.width() as f32;
    let img_big_w = 500;
    let img_big_h = (img_big_w as f32 * ratio).round() as i32;
    let img_small_w = 100;
    let img_small_h = (img_small_w as f32 * ratio).round() as i32;
    let h1 = img_big_h;
    let h2 = img_small_h.max(80);
    let frame_w = img_big_w;
    let frame_h = h1 + h2 + 10;

    let mut surface = new_surface((frame_w, frame_h));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);

    canvas
        .draw_text_area_auto_font_size(
            IRect::from_ltrb(20, h1 + 5, 280, frame_h - 5),
            "要我一直",
            20.0,
            60.0,
            text_params!(text_align = TextAlign::Right),
        )
        .unwrap();
    canvas
        .draw_text_area_auto_font_size(
            IRect::from_ltrb(400, h1 + 5, 480, frame_h - 5),
            "吗",
            20.0,
            60.0,
            text_params!(text_align = TextAlign::Left),
        )
        .unwrap();
    let frame = surface.image_snapshot();

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image_big = images[0].resize_width(img_big_w);
        let image_small = images[0].resize_width(img_small_w);
        canvas.draw_image(&image_big, (0, 0), None);
        canvas.draw_image(
            &image_small,
            (290, h1 + 5 + (h2 - image_small.height()) / 2),
            None,
        );
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

fn always_always(images: Vec<InputImage>, loop_: bool) -> Result<Vec<u8>, Error> {
    let img = &images[0];
    let ratio = img.image.height() as f32 / img.image.width() as f32;
    let img_big_w = 500;
    let img_big_h = (img_big_w as f32 * ratio).round() as i32;
    let img_small_w = 100;
    let img_small_h = (img_small_w as f32 * ratio).round() as i32;
    let img_tiny_w = 20;
    let img_tiny_h = (img_tiny_w as f32 * ratio).round() as i32;
    let text_h = img_small_h + img_tiny_h + 10;
    let text_h = text_h.max(80);
    let frame_w = img_big_w;
    let frame_h = img_big_h + text_h;

    let mut surface = new_surface((frame_w, frame_h));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);

    canvas
        .draw_text_area_auto_font_size(
            IRect::from_ltrb(20, img_big_h + 5, 280, frame_h - 5),
            "要我一直",
            20.0,
            60.0,
            text_params!(text_align = TextAlign::Right),
        )
        .unwrap();
    canvas
        .draw_text_area_auto_font_size(
            IRect::from_ltrb(400, img_big_h + 5, 480, frame_h - 5),
            "吗",
            20.0,
            60.0,
            text_params!(text_align = TextAlign::Left),
        )
        .unwrap();
    let text_frame = surface.image_snapshot();

    let frame_num = 20;
    let coeff = (5.0_f32).powf(1.0 / frame_num as f32);

    let func = |i: usize, images: Vec<Image>| {
        let mut surface = text_frame.to_surface();
        let canvas = surface.canvas();
        let image = images[0].resize_width(img_big_w);
        canvas.draw_image(&image, (0, 0), None);
        let base_frame = surface.image_snapshot();

        let mut surface = new_surface((frame_w, frame_h));
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let mut r = coeff.powi(i as i32);
        for _ in 0..4 {
            let x = (358.0 * (1.0 - r)).round() as i32;
            let y = (frame_h as f32 * (1.0 - r)).round() as i32;
            let w = (frame_w as f32 * r).round() as i32;
            let h = (frame_h as f32 * r).round() as i32;
            let image = base_frame.resize_exact((w, h));
            canvas.draw_image(&image, (x, y), None);
            r /= 5.0;
        }

        Ok(surface.image_snapshot())
    };

    if !loop_ {
        return make_png_or_gif(images, |images: Vec<Image>| func(0, images));
    }

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num,
            duration: 0.1,
        },
        FrameAlign::ExtendLoop,
    )
}

fn always(images: Vec<InputImage>, _: Vec<String>, options: Mode) -> Result<Vec<u8>, Error> {
    let mode = if options.circle.unwrap() {
        "circle"
    } else if options.r#loop.unwrap() {
        "loop"
    } else {
        options.mode.as_deref().unwrap()
    };

    match mode {
        "circle" => always_always(images, false),
        "loop" => always_always(images, true),
        _ => always_normal(images),
    }
}

register_meme!(
    "always",
    always,
    min_images = 1,
    max_images = 1,
    keywords = &["一直"],
    shortcuts = &[shortcut!("一直一直", options = &[("mode", "loop")])],
    date_created = local_date(2021, 12, 2),
    date_modified = local_date(2024, 8, 9),
);
