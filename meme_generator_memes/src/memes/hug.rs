use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn hug(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let user_locs = [
        (108, 15),
        (107, 14),
        (104, 16),
        (102, 14),
        (104, 15),
        (108, 15),
        (108, 15),
        (103, 16),
        (102, 15),
        (104, 14),
    ];
    let self_locs = [
        (78, 120),
        (115, 130),
        (0, 0),
        (110, 100),
        (80, 100),
        (75, 115),
        (105, 127),
        (0, 0),
        (110, 98),
        (80, 105),
    ];
    let angles = [48, 18, 0, -38, -31, 43, 22, 0, -34, -35];
    let self_head = images[0].image.square().resize_exact((120, 120));
    let user_head = images[1].image.square().resize_exact((105, 105));

    let mut encoder = GifEncoder::new();
    for i in 0..10 {
        let frame = load_image(format!("hug/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);

        canvas.draw_image(&user_head, user_locs[i], None);
        canvas.draw_image(&self_head.rotate(angles[i] as f32), self_locs[i], None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.05)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "hug",
    hug,
    min_images = 2,
    max_images = 2,
    keywords = &["抱", "抱抱"],
    date_created = local_date(2024, 8, 6),
    date_modified = local_date(2024, 8, 6),
);
