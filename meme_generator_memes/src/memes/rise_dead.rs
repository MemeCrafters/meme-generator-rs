use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn rise_dead(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let params = [
        ((81, 55), [(0, 2), (101, 0), (103, 105), (1, 105)]),
        ((74, 49), [(0, 3), (104, 0), (106, 108), (1, 108)]),
        ((-66, 36), [(0, 0), (182, 5), (184, 194), (1, 185)]),
        ((-231, 55), [(0, 0), (259, 4), (276, 281), (13, 278)]),
    ];
    let img = images[0].image.square().resize_exact((150, 150));

    let mut encoder = GifEncoder::new();
    for i in 0..34 {
        let frame = load_image(format!("rise_dead/{i:02}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);

        if i <= 28 {
            let idx = if i <= 25 { 0 } else { i - 25 };
            let (loc, points) = params[idx];
            let (x, y) = loc;
            let (x, y) = if i % 2 == 1 { (x + 1, y - 1) } else { (x, y) };
            let img = img.perspective(&points);
            canvas.draw_image(&img, (x, y), None);
        }

        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.15)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "rise_dead",
    rise_dead,
    min_images = 1,
    max_images = 1,
    keywords = &["诈尸", "秽土转生"],
    date_created = local_date(2022, 11, 8),
    date_modified = local_date(2023, 2, 14),
);
