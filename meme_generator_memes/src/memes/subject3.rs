use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn subject3(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let locs = [
        (60, 71),
        (61, 73),
        (62, 71),
        (66, 70),
        (75, 69),
        (87, 74),
        (87, 74),
        (85, 76),
        (79, 73),
        (76, 71),
        (68, 69),
        (66, 73),
        (66, 74),
        (66, 74),
        (66, 71),
        (76, 65),
        (80, 65),
        (91, 73),
        (91, 77),
        (91, 75),
        (86, 71),
        (83, 69),
        (78, 68),
        (73, 67),
        (68, 74),
        (68, 77),
        (71, 73),
        (81, 68),
        (88, 69),
        (96, 73),
        (98, 78),
        (97, 79),
        (93, 76),
        (85, 71),
        (80, 66),
        (71, 69),
        (69, 74),
        (68, 77),
        (68, 77),
        (80, 70),
        (91, 68),
        (95, 71),
        (98, 78),
        (97, 79),
        (95, 78),
        (86, 69),
        (77, 64),
        (71, 69),
        (71, 73),
        (69, 73),
        (73, 67),
        (78, 65),
        (88, 65),
        (91, 72),
        (94, 77),
        (91, 74),
        (89, 70),
        (83, 63),
        (75, 60),
        (69, 67),
        (67, 74),
        (68, 73),
        (76, 64),
        (77, 60),
        (84, 62),
        (92, 68),
        (92, 73),
        (90, 69),
        (86, 66),
        (80, 61),
        (69, 63),
        (65, 67),
        (60, 76),
        (62, 73),
        (66, 68),
        (75, 62),
        (77, 62),
        (85, 69),
        (86, 73),
        (85, 75),
        (78, 70),
        (74, 67),
        (67, 67),
        (65, 72),
        (65, 79),
    ];
    let img = images[0].image.circle().resize_exact((120, 120));

    let mut encoder = GifEncoder::new();
    for i in 0..85 {
        let frame = load_image(format!("subject3/{i:02}.png"))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        canvas.draw_image(&img, locs[i], None);
        encoder.add_frame(surface.image_snapshot(), 0.08)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "subject3",
    subject3,
    min_images = 1,
    max_images = 1,
    keywords = &["科目三"],
    date_created = local_date(2024, 4, 17),
    date_modified = local_date(2024, 4, 17),
);
