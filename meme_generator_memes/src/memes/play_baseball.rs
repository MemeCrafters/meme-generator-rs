use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn play_baseball(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let img = images[0].image.circle().resize_exact((150, 150));
    let mut ball_x = 0;
    let mut ball_v = 40;
    let mut ball_angle = 0;

    let mut encoder = GifEncoder::new();
    for i in 0..20 {
        let mut surface = new_surface((1000, 300));
        let canvas = surface.canvas();
        ball_x += ball_v;
        ball_angle += if ball_v > 0 { 60 } else { -60 };
        if ball_x >= 200 {
            ball_x = 200;
            ball_v = -ball_v;
        } else if ball_x <= -200 {
            ball_x = -200;
            ball_v = -ball_v;
        }
        canvas.draw_image(
            &img.rotate_crop(ball_angle as f32),
            (425 + ball_x, 120),
            None,
        );
        let right_index = if i >= 3 && i <= 7 { i - 2 } else { 0 };
        let left_index = if i >= 13 && i <= 17 { i - 12 } else { 0 };
        let right = load_image(format!("play_baseball/{right_index}.png"))?;
        let left = load_image(format!("play_baseball/{left_index}.png"))?.flip_horizontal();
        canvas.draw_image(&right, (630, 6), None);
        canvas.draw_image(&left, (0, 6), None);
        encoder.add_frame(surface.image_snapshot(), 0.08)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "play_baseball",
    play_baseball,
    min_images = 1,
    max_images = 1,
    keywords = &["打棒球"],
    tags = MemeTags::capoo(),
    date_created = local_date(2025, 6, 3),
    date_modified = local_date(2025, 6, 3),
);
