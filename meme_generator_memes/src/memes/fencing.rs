use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn fencing(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let user_locs = [
        (57, 4),
        (55, 5),
        (58, 7),
        (57, 5),
        (53, 8),
        (54, 9),
        (64, 5),
        (66, 8),
        (70, 9),
        (73, 8),
        (81, 10),
        (77, 10),
        (72, 4),
        (79, 8),
        (50, 8),
        (60, 7),
        (67, 6),
        (60, 6),
        (50, 9),
    ];
    let self_locs = [
        (10, 6),
        (3, 6),
        (32, 7),
        (22, 7),
        (13, 4),
        (21, 6),
        (30, 6),
        (22, 2),
        (22, 3),
        (26, 8),
        (23, 8),
        (27, 10),
        (30, 9),
        (17, 6),
        (12, 8),
        (11, 7),
        (8, 6),
        (-2, 10),
        (4, 9),
    ];
    let self_head = images[0].image.circle().resize_exact((27, 27));
    let user_head = images[1].image.circle().resize_exact((27, 27));

    let mut encoder = GifEncoder::new();
    for i in 0..19 {
        let frame = load_image(format!("fencing/{i:02}.png"))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();

        canvas.draw_image(&user_head, user_locs[i], None);
        canvas.draw_image(&self_head, self_locs[i], None);
        encoder.add_frame(surface.image_snapshot(), 0.05)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "fencing",
    fencing,
    min_images = 2,
    max_images = 2,
    keywords = &["击剑", "🤺"],
    date_created = local_date(2022, 10, 1),
    date_modified = local_date(2023, 2, 14),
);
