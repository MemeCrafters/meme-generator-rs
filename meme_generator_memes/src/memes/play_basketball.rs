use skia_safe::ISize;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::GifEncoder,
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme, tags::MemeTags};

fn play_basketball(
    images: Vec<InputImage>,
    _: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let locs = [
        Some((297, 321, 0.0)),
        Some((300, 327, 7.2)),
        Some((308, 410, 5.0)),
        Some((308, 412, 5.0)),
        Some((301, 355, 0.0)),
        Some((296, 317, 0.0)),
        Some((296, 320, 0.0)),
        Some((296, 320, 0.0)),
        Some((352, 435, 0.0)),
        None,
        None,
        None,
        None,
        Some((175, 343, 5.0)),
        Some((173, 353, 5.0)),
        Some((173, 353, 2.0)),
        Some((171, 354, 2.0)),
        Some((189, 158, 0.0)),
        Some((213, 149, 0.0)),
        Some((238, 139, 0.0)),
        Some((245, 139, 0.0)),
        Some((252, 139, -12.0)),
        Some((257, 142, -12.0)),
        Some((261, 142, -17.0)),
        Some((265, 145, -17.0)),
        Some((271, 148, -18.0)),
        Some((279, 156, -18.0)),
        Some((286, 163, -25.0)),
        Some((287, 160, -25.0)),
        Some((289, 159, -27.0)),
        Some((286, 165, -27.0)),
        Some((285, 167, -20.0)),
        Some((285, 179, -20.0)),
        Some((282, 192, 25.0)),
        Some((284, 219, 25.0)),
        Some((280, 242, 32.0)),
        Some((283, 280, 32.0)),
        Some((287, 315, 32.0)),
    ];
    let img = images[0].image.circle().resize_exact((77, 77));

    let mut encoder = GifEncoder::new();
    for i in 0..38 {
        let frame = load_image(format!("play_basketball/{i:02}.png"))?;
        let frame = if let Some((x, y, angle)) = locs[i] {
            let mut surface = new_surface(frame.dimensions());
            let canvas = surface.canvas();
            let img = img.rotate(angle);
            let ISize { width, height } = img.dimensions();
            canvas.draw_image(&img, (x - width / 2, y - height / 2), None);
            canvas.draw_image(&frame, (0, 0), None);
            surface.image_snapshot()
        } else {
            frame
        };
        encoder.add_frame(frame, 0.08)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "play_basketball",
    play_basketball,
    min_images = 1,
    max_images = 1,
    keywords = &["打篮球", "火柴人打篮球"],
    tags = MemeTags::stickman(),
    date_created = local_date(2025, 4, 30),
    date_modified = local_date(2025, 4, 30),
);
