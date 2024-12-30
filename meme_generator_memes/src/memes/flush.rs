use rand::Rng;
use skia_safe::{Color, Color4f, IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    encoder::{make_gif_or_combined_gif, GifInfo},
    image::ImageExt,
    tools::{load_image, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn flush(images: Vec<DecodedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |i: usize, images: &Vec<Image>| {
        let mut image = images[0].square();
        let max_width = 282;
        if image.width() > max_width {
            image = image.resize_exact((max_width, max_width));
        }
        let (w, h) = (image.width(), image.height());

        if i >= 18 {
            let frame = load_image(format!("flush/{:02}.png", i - 18))?;
            return Ok(frame.resize_exact((w, h)));
        }

        let mut rng = rand::thread_rng();
        let padding_ratio = 0.01 * i as f32;
        let jitter_ratio = padding_ratio * 0.4 * rng.gen_range(-0.5..0.5);
        let padding = (w as f32 * padding_ratio).round() as i32;
        let jitter = (w as f32 * jitter_ratio).round() as i32;
        let alpha = i as f32 * 0.03;
        let crop_box = IRect::new(
            padding + jitter,
            padding + jitter,
            w - padding + jitter,
            h - padding + jitter,
        );
        let cropped_img = image.crop(crop_box);
        let mut surface = new_surface(cropped_img.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        canvas.draw_image(&cropped_img, (0, 0), None);
        let red_color = Color4f::new(1.0, 0.0, 0.0, alpha);
        canvas.draw_irect(cropped_img.bounds(), &new_paint(red_color));
        Ok(surface.image_snapshot().resize_exact((w, h)))
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 30,
            duration: 0.08,
        },
        None,
    )
}

register_meme!(
    "flush",
    flush,
    min_images = 1,
    max_images = 1,
    keywords = &["红温"],
    date_created = local_date(2024, 9, 3),
    date_modified = local_date(2024, 9, 3),
);
