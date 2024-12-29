use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{local_date, new_surface},
};

use crate::{options::Circle, register_meme};

fn kaleidoscope(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    options: &Circle,
) -> Result<Vec<u8>, Error> {
    let circle_num = 10;
    let img_per_circle = 4;
    let init_angle = 0.0;
    let angle_step = 360.0 / img_per_circle as f32;

    let radius = |n: usize| -> f32 { n as f32 * 50.0 + 100.0 };

    let cx = radius(circle_num) as f32;
    let cy = cx;

    let func = |images: &Vec<Image>| {
        let mut surface = new_surface(((cx * 2.0) as i32, (cy * 2.0) as i32));
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);

        let image = &images[0];
        let mut current_angle = init_angle;

        for i in 0..circle_num {
            let r = radius(i);
            let img_w = (i * 35 + 100) as i32;
            let mut img = image.resize_width(img_w);
            if options.circle {
                img = img.circle();
            }

            for j in 0..img_per_circle {
                let angle = current_angle + angle_step * j as f32;
                let img_rot = img.rotate(90.0 - angle);
                let x = (cx + r * angle.to_radians().cos() - img_rot.width() as f32 / 2.0).round()
                    as i32;
                let y = (cy - r * angle.to_radians().sin() - img_rot.height() as f32 / 2.0).round()
                    as i32;
                canvas.draw_image(&img_rot, (x, y), None);
            }
            current_angle += angle_step / 2.0;
        }

        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "kaleidoscope",
    kaleidoscope,
    min_images = 1,
    max_images = 1,
    keywords = &["万花筒", "万花镜"],
    date_created = local_date(2023, 1, 8),
    date_modified = local_date(2023, 2, 14),
);
