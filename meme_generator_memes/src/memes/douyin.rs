use rand::Rng;
use skia_safe::{Color, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::encode_gif,
    image::ImageExt,
    text::Text2Image,
    text_params,
    tools::{color_from_hex_code, local_date, new_paint, new_stroke_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn douyin(_: Vec<NamedImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let fontsize = 200.0;
    let offset = (fontsize * 0.05) as i32;
    let padding_x = 70;
    let padding_y = 30;
    let bg_color = color_from_hex_code("#1C0B1B");

    let text_right = Text2Image::from_text(
        text,
        fontsize,
        text_params!(
            paint = new_paint(color_from_hex_code("#FF0050")),
            stroke_paint = new_stroke_paint(color_from_hex_code("#FF0050"), 10.0),
        ),
    );
    let text_left = Text2Image::from_text(
        text,
        fontsize,
        text_params!(
            paint = new_paint(color_from_hex_code("#00F5EB")),
            stroke_paint = new_stroke_paint(color_from_hex_code("#00F5EB"), 10.0),
        ),
    );
    let text_mid = Text2Image::from_text(
        text,
        fontsize,
        text_params!(
            paint = new_paint(Color::WHITE),
            stroke_paint = new_stroke_paint(Color::WHITE, 10.0),
        ),
    );

    let frame_w = text_mid.longest_line().ceil() as i32 + padding_x * 2 + offset * 2;
    let frame_h = text_mid.height().ceil() as i32 + padding_y * 2 + offset * 2;
    let mut surface = new_surface((frame_w, frame_h));
    let mut canvas = surface.canvas();
    canvas.clear(bg_color);
    text_right.draw_on_canvas(
        &mut canvas,
        (padding_x + offset * 2, padding_y + offset * 2),
    );
    text_left.draw_on_canvas(&mut canvas, (padding_x, padding_y));
    text_mid.draw_on_canvas(&mut canvas, (padding_x + offset, padding_y + offset));
    let frame = surface.image_snapshot();

    let width = frame_w - padding_x * 2;
    let height = frame_h - padding_y * 2;
    let frame_num = 10;
    let devide_num = 6;
    let seed = 20.0 * 0.05;
    let tilt = 0.17;
    let mut frames: Vec<Image> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..frame_num {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();

        let h_seeds: Vec<f32> = (0..devide_num)
            .map(|_| rng.gen_range(0.0..devide_num as f32).sin().abs())
            .collect();
        let h_seed_sum: f32 = h_seeds.iter().sum();
        let h_seeds: Vec<f32> = h_seeds.iter().map(|s| s / h_seed_sum).collect();

        let mut direction = 1;
        let mut last_yn = 0;
        let mut last_h = 0;

        for i in 0..devide_num {
            let yn = last_yn + last_h;
            let h = (height as f32 * h_seeds[i as usize]).round().max(2.0) as i32;
            last_yn = yn;
            last_h = h;
            direction = -direction;
            let piece = frame.crop(IRect::from_ltrb(
                padding_x,
                padding_y + yn,
                padding_x + width,
                padding_y + yn + h,
            ));
            canvas.draw_image(
                &piece,
                (padding_x + (i * direction * seed as i32), padding_y + yn),
                None,
            );
        }
        let frame = surface.image_snapshot();

        let move_x = (frame_h as f32 * tilt).round() as i32;
        let points = [
            (move_x, 0),
            (frame_w + move_x, 0),
            (frame_w, frame_h),
            (0, frame_h),
        ];
        let frame = frame.perspective(&points);

        frames.push(frame.with_background(bg_color));
    }

    encode_gif(frames, 0.2)
}

register_meme!(
    "douyin",
    douyin,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["douyin"],
    keywords = &["douyin"],
    date_created = local_date(2022, 10, 29),
    date_modified = local_date(2023, 2, 14),
);
