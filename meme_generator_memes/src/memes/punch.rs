use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn punch(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (-50, 20),
        (-40, 10),
        (-30, 0),
        (-20, -10),
        (-10, -10),
        (0, 0),
        (10, 10),
        (20, 20),
        (10, 10),
        (0, 0),
        (-10, -10),
        (10, 0),
        (-30, 10),
    ];
    let img = images[0].image.square().resize_exact((260, 260));

    let mut encoder = GifEncoder::new();
    for i in 0..13 {
        let frame = load_image(format!("punch/{i:02}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        let (x, y) = locs[i];
        canvas.draw_image(&img, (x, y - 15), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.03)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "punch",
    punch,
    min_images = 1,
    max_images = 1,
    keywords = &["打拳"],
    date_created = local_date(2022, 3, 18),
    date_modified = local_date(2023, 2, 14),
);
