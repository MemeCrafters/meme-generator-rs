use rand::RngExt;
use skia_safe::{Canvas, Color, IRect, Image, Point};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_paint},
};

use crate::{options::NoOptions, register_meme};

#[derive(Copy, Clone, Debug)]
struct Bucket {
    r: f32,
    g: f32,
    b: f32,
    count: f32,
}

pub fn get_dominant_colors(image: &Image) -> Vec<Color> {
    let image = if image.width() > 200 || image.height() > 200 {
        &image.resize_bound((200, 200), Fit::Contain)
    } else {
        image
    };
    let pixmap = image.peek_pixels().unwrap();
    let mut buckets = [[[Bucket {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        count: 0.0,
    }; 2]; 2]; 2];
    let mut sampled_pixel_count = 0.0;
    for x in 0..pixmap.width() {
        for y in 0..pixmap.height() {
            let color = pixmap.get_color((x, y));
            let r = color.r();
            let g = color.g();
            let b = color.b();
            let i = (r >> 7) as usize;
            let j = (g >> 7) as usize;
            let k = (b >> 7) as usize;
            buckets[i][j][k].r += r as f32;
            buckets[i][j][k].g += g as f32;
            buckets[i][j][k].b += b as f32;
            buckets[i][j][k].count += 1.0;
            sampled_pixel_count += 1.0;
        }
    }

    let mut buckets_averages: Vec<Bucket> = Vec::new();
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let current_bucket = &buckets[i][j][k];
                if current_bucket.count > 0.0 {
                    buckets_averages.push(Bucket {
                        r: current_bucket.r / current_bucket.count,
                        g: current_bucket.g / current_bucket.count,
                        b: current_bucket.b / current_bucket.count,
                        count: current_bucket.count,
                    })
                }
            }
        }
    }

    buckets_averages.sort_by(|a, b| b.count.partial_cmp(&a.count).unwrap());

    let mut colors: Vec<Color> = Vec::new();
    for ba in &buckets_averages {
        if ba.count / sampled_pixel_count > 0.01 {
            colors.push(Color::from_rgb(
                ba.r.round() as u8,
                ba.g.round() as u8,
                ba.b.round() as u8,
            ));
        }
    }

    colors
}

fn draw_random_blocks(canvas: &Canvas, colors: &Vec<Color>, mask: &Image) {
    let (x1, y1, x2, y2) = (200, 300, 400, 650);
    let mut block_locs: Vec<(i32, i32)> = Vec::new();
    let mut rng = rand::rng();
    let mask_pixmap = mask.peek_pixels().unwrap();
    for _ in 0..150 {
        let x = rng.random_range(x1..=x2);
        let y = rng.random_range(y1..=y2);
        if mask_pixmap.get_color((x, y)) == Color::BLACK {
            continue;
        }
        if block_locs
            .iter()
            .any(|&(x_, y_)| (x - x_).abs() < 13 && (y - y_).abs() < 13)
        {
            continue;
        }
        block_locs.push((x, y));
        let color = colors[rng.random_range(0..colors.len())];
        canvas.rotate(45.0, Some(Point::new(x as f32, y as f32)));
        canvas.draw_irect(IRect::from_xywh(x, y, 10, 10), &new_paint(color));
        canvas.reset_matrix();
    }
}

fn dont_touch(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("dont_touch/0.png")?;
    let mask = load_image("dont_touch/mask.png")?;
    let mask = mask.make_raster_image(None, None).unwrap();

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let img = images[0].resize_fit((250, 250), Fit::Contain);
        let colors = get_dominant_colors(&img);
        draw_random_blocks(&canvas, &colors, &mask);
        canvas.draw_image(&img, (25, 460), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "dont_touch",
    dont_touch,
    min_images = 1,
    max_images = 1,
    keywords = &["别碰"],
    date_created = local_date(2023, 4, 27),
    date_modified = local_date(2023, 4, 27),
);
