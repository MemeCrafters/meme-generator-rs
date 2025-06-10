use skia_safe::Color;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn thump_wildly(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let img = images[0].image.square().resize_exact((122, 122));

    let mut encoder = GifEncoder::new();
    for i in 0..37 {
        let i = if i >= 31 { 0 } else { i };
        let frame = load_image(format!("thump_wildly/{i:02}.png"))?;

        let frame = if (15..31).contains(&i) {
            frame
        } else {
            let mut surface = new_surface(frame.dimensions());
            let canvas = surface.canvas();
            canvas.clear(Color::WHITE);
            let pos = if i == 14 { (207, 239) } else { (203, 196) };
            canvas.draw_image(&img, pos, None);
            canvas.draw_image(&frame, (0, 0), None);
            surface.image_snapshot()
        };
        encoder.add_frame(frame, 0.04)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "thump_wildly",
    thump_wildly,
    min_images = 1,
    max_images = 1,
    tags = MemeTags::arknights(),
    keywords = &["捶爆", "爆捶"],
    date_created = local_date(2023, 3, 31),
    date_modified = local_date(2023, 3, 31),
);
