use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags, union_tags};

fn beat_up(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let self_locs = [(100, 43), (110, 46), (101, 40)];
    let user_locs = [(99, 136), (99, 136), (89, 140)];
    let self_head = images[0].image.circle().resize_exact((55, 55));
    let user_head = images[1].image.circle().resize_exact((45, 45));

    let mut encoder = GifEncoder::new();
    for i in 0..3 {
        let frame = load_image(&format!("beat_up/{i}.png"))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        canvas.draw_image(&self_head, self_locs[i], None);
        canvas.draw_image(&user_head, user_locs[i], None);
        encoder.add_frame(surface.image_snapshot(), 0.1)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "beat_up",
    beat_up,
    min_images = 2,
    max_images = 2,
    keywords = &["揍"],
    tags = union_tags!(MemeTags::tom(), MemeTags::jerry()),
    date_created = local_date(2024, 4, 9),
    date_modified = local_date(2024, 4, 9),
);
