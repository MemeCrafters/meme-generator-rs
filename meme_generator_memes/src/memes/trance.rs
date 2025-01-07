use skia_safe::{IRect, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn trance(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let func = |images: Vec<Image>| {
        let img = &images[0];
        let img_w = img.width();
        let img_h = img.height();
        let dh = (img_h as f32 * 0.1) as i32;
        let frame_w = img_w;
        let frame_h = img_h + dh;
        let mut surface = new_surface((frame_w, frame_h));
        let canvas = surface.canvas();
        canvas.draw_image(img, (0, dh), None);
        let img = img.transparency(0.01);
        for i in (0..dh).rev() {
            canvas.draw_image(&img, (0, i), None);
        }
        for i in dh..dh * 2 {
            canvas.draw_image(&img, (0, i), None);
        }

        let frame = surface.image_snapshot();
        let frame = frame.crop(IRect::from_ltrb(0, dh, frame_w, frame_h));
        Ok(frame)
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "trance",
    trance,
    min_images = 1,
    max_images = 1,
    keywords = &["恍惚"],
    date_created = local_date(2022, 12, 11),
    date_modified = local_date(2023, 2, 14),
);
