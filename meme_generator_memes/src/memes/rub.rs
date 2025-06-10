use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn rub(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let user_locs = [
        (39, 91, 75, 75),
        (49, 101, 75, 75),
        (67, 98, 75, 75),
        (55, 86, 75, 75),
        (61, 109, 75, 75),
        (65, 101, 75, 75),
    ];
    let self_locs = [
        (102, 95, 70, 80, 0),
        (108, 60, 50, 100, 0),
        (97, 18, 65, 95, 0),
        (65, 5, 75, 75, 20),
        (95, 57, 100, 55, 70),
        (109, 107, 65, 75, 0),
    ];
    let user_head = images[1].image.circle();
    let self_head = images[0].image.circle();

    let mut encoder = GifEncoder::new();
    for i in 0..6 {
        let frame = load_image(format!("rub/{i}.png"))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();

        let (x, y, w, h) = user_locs[i];
        canvas.draw_image(&user_head.resize_exact((w, h)), (x, y), None);

        let (x, y, w, h, a) = self_locs[i];
        canvas.draw_image(
            &self_head.resize_exact((w, h)).rotate(a as f32),
            (x, y),
            None,
        );

        encoder.add_frame(surface.image_snapshot(), 0.05)?;
    }
    Ok(encoder.finish()?)
}

register_meme! {
    "rub",
    rub,
    min_images = 2,
    max_images = 2,
    keywords = &["贴", "贴贴", "蹭", "蹭蹭"],
    date_created = local_date(2021, 6, 11),
    date_modified = local_date(2023, 2, 14),
}
