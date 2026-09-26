use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn zzdd(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (47, 63),
        (47, 63),
        (47, 63),
        (47, 63),
        (51, 126),
        (51, 126),
        (51, 126),
        (51, 126),
        (51, 126),
        (39, 105),
        (39, 105),
        (39, 105),
        (39, 105),
        (39, 105),
        (47, 63),
    ];
    let image = images[0].image.resize_exact((80, 80));

    let mut encoder = GifEncoder::new();
    for (i, &(x, y)) in locs.iter().enumerate() {
        let frame = load_image(format!("zzdd/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&image, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        encoder.add_frame(surface.image_snapshot(), 0.04)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "zzdd",
    zzdd,
    min_images = 1,
    max_images = 1,
    keywords = &["指指点点"],
    date_created = local_date(2025, 7, 3),
    date_modified = local_date(2025, 7, 3),
);
