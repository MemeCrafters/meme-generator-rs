use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn printing(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let img = images[0].image.resize_bound((304, 174), Fit::Contain);

    let mut encoder = GifEncoder::new();
    for i in 0..115 {
        let frame = load_image(format!("printing/{i:03}.png"))?;
        let frame = if (50..115).contains(&i) {
            let mut surface = new_surface(frame.dimensions());
            let canvas = surface.canvas();
            canvas.clear(Color::WHITE);
            canvas.draw_image(&img, (298 - img.width() / 2, 339 - img.height()), None);
            canvas.draw_image(&frame, (0, 0), None);
            surface.image_snapshot()
        } else {
            frame
        };
        encoder.add_frame(frame, 0.05)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "printing",
    printing,
    min_images = 1,
    max_images = 1,
    keywords = &["打印"],
    date_created = local_date(2023, 1, 26),
    date_modified = local_date(2023, 2, 14),
);
