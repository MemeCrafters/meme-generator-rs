use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn capoo_rub(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (178, 184, 78, 260),
        (178, 174, 84, 269),
        (178, 174, 84, 269),
        (178, 178, 84, 264),
    ];
    let image = images[0].image.square();

    let mut encoder = GifEncoder::new();
    for i in 0..4 {
        let (w, h, x, y) = locs[i];
        let frame = load_image(format!("capoo_rub/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = image.resize_exact((w, h));
        canvas.draw_image(&image, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.1)?;
    }
    Ok(encoder.finish()?)
}

register_meme! {
    "capoo_rub",
    capoo_rub,
    min_images = 1,
    max_images = 1,
    keywords = &["咖波蹭", "咖波贴"],
    tags = MemeTags::capoo(),
    date_created = local_date(2022, 11, 29),
    date_modified = local_date(2023, 2, 14),
}
