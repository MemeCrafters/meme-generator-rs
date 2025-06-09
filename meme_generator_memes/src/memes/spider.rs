use rand::Rng;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn spider(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let xs = [
        174, 174, 174, 169, 165, 160, 154, 150, 144, 141, 137, 133, 130, 119, 115, 113, 108, 103,
        103, 97, 91, 85, 87, 79, 74, 79, 75, 75, 78, 79, 77, 77, 70, 81, 93, 94, 104, 110, 119,
        123, 131, 134, 143, 154, 158, 161, 163, 169, 174, 173, 174, 173,
    ];
    let img = images[0].image.circle().resize_exact((80, 80));
    let mut rng = rand::thread_rng();

    let mut encoder = GifEncoder::new();
    for i in 0..52 {
        let frame = load_image(format!("spider/{i:02}.png"))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        canvas.draw_image(&img, (xs[i], 24 + rng.gen_range(-1..=1)), None);
        encoder.add_frame(surface.image_snapshot(), 0.04)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "spider",
    spider,
    min_images = 1,
    max_images = 1,
    keywords = &["蜘蛛", "蜘蛛爬"],
    date_created = local_date(2025, 4, 27),
    date_modified = local_date(2025, 4, 27),
);
