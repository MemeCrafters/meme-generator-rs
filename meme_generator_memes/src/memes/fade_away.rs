use rand::Rng;
use skia_safe::{Canvas, Color, Data, Image, Paint, Rect, RuntimeEffect};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{default_sampling_options, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

struct Dot {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    dx: f32,
    dy: f32,
    radius: f32,
    out_of_rect: bool,
}

impl Dot {
    fn new(positon: (f32, f32), direction: (f32, f32)) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: positon.0,
            y: positon.1,
            vx: 0.0,
            vy: 0.0,
            dx: direction.0,
            dy: direction.1,
            radius: rng.gen_range(1.0..=3.0),
            out_of_rect: false,
        }
    }

    fn move_step(&mut self, step: f32) {
        let a = 0.02 * step / self.radius;
        self.vx += a * self.dx;
        self.vy += a * self.dy;
        self.x += self.vx;
        self.y += self.vy;
        let mut rng = rand::thread_rng();
        if rng.gen_range(0.0..1.0) < 0.25 {
            self.radius -= 1.0;
        }
    }

    fn out_of_rect(&self, rect: &Rect) -> bool {
        self.radius <= 0.0
            || self.x + self.radius < rect.left()
            || self.x - self.radius > rect.right()
            || self.y + self.radius < rect.top()
            || self.y - self.radius > rect.bottom()
    }

    fn draw_on(&self, canvas: &Canvas) {
        let paint = new_paint(Color::BLACK);
        canvas.draw_circle((self.x, self.y), self.radius, &paint);
    }
}

fn draw_dusts(canvas: &Canvas, dusts: &mut Vec<Dot>, step: f32) {
    for dot in dusts.iter_mut() {
        if dot.out_of_rect {
            continue;
        }
        dot.move_step(step);
        if dot.out_of_rect(&Rect::from_irect(canvas.image_info().bounds())) {
            dot.out_of_rect = true;
        } else {
            dot.draw_on(canvas);
        }
    }
}

fn fade_away(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let sksl_code = r#"
        uniform shader image;
        uniform float2 center;
        uniform float r1;
        uniform float r2;
        uniform float r3;

        half4 main(float2 coord) {
            half4 color = image.eval(coord);
            if (color.a == 0) {
                return color;
            }
            float distance = length(coord - center);
            if (distance <= r1) {
                return half4(0.0);
            } else if (distance <= r2) {
                return half4(0.0, 0.0, 0.0, 1.0);
            } else if (distance <= r3) {
                float factor = 0.5 + (distance - r2) / (r3 - r2);
                factor = clamp(factor, 0.0, 1.0);
                float rand = fract(sin(dot(coord, float2(12.9898, 78.233))) * 43758.5453);
                factor *= 0.9 + 0.2 * rand;
                float gray = (color.r + color.g + color.b) / 3.0;
                gray *= factor;
                return half4(gray, gray, gray, color.a);
            } else {
                return color;
            }
        }
    "#;
    let effect = RuntimeEffect::make_for_shader(sksl_code, None).unwrap();

    let mut dusts = Vec::new();

    let func = |i: usize, images: Vec<Image>| {
        let img = &images[0];
        let max_width = 200;
        let img = if img.width() > max_width {
            &img.resize_width(max_width)
        } else {
            img
        };
        let img_w = img.width();
        let img_h = img.height();

        let center_x = img_w as f32 * 2.0 / 3.0;
        let center_y = img_h as f32 * 3.0 / 2.0;
        let radius = (center_x * center_x + center_y * center_y).sqrt();
        let step = radius / 24.0;

        if i <= 9 {
            Ok(img.clone())
        } else if 9 < i && i < 28 {
            let t = i - 9;
            let r1 = step * (t + 4) as f32;
            let r2 = step * (t + 5) as f32;
            let r3 = step * (t + 11) as f32;

            let mut values = Vec::new();
            for uniform in effect.uniforms() {
                match uniform.name() {
                    "center" => {
                        values.extend(center_x.to_le_bytes());
                        values.extend(center_y.to_le_bytes());
                    }
                    "r1" => values.extend(r1.to_le_bytes()),
                    "r2" => values.extend(r2.to_le_bytes()),
                    "r3" => values.extend(r3.to_le_bytes()),
                    _ => {}
                }
            }
            let uniforms = Data::new_copy(&values);

            let image_shader = img
                .to_shader(None, default_sampling_options(), None)
                .unwrap();
            let shader = effect
                .make_shader(&uniforms, &[image_shader.into()], None)
                .unwrap();

            let mut surface = new_surface(img.dimensions());
            let canvas = surface.canvas();
            let mut paint = Paint::default();
            paint.set_shader(shader);
            canvas.draw_paint(&paint);

            let mut rng = rand::thread_rng();
            let pixmap = img.peek_pixels().unwrap();
            for r in r1 as i32..r2 as i32 {
                for theta in 0..180 {
                    let theta = (theta as f32).to_radians();
                    let x = (center_x + r as f32 * theta.cos()) as i32;
                    let y = (center_y - r as f32 * theta.sin()) as i32;
                    if x < 0 || x >= img_w || y < 0 || y >= img_h {
                        continue;
                    }
                    let color = pixmap.get_color((x, y));
                    if color.a() == 0 {
                        continue;
                    }
                    if rng.gen_range(0.0..1.0) < 0.1 {
                        let dx = (x as f32 - center_x) / r as f32;
                        let dy = 1.5 * (y as f32 - center_y) / r as f32;
                        dusts.push(Dot::new((x as f32, y as f32), (dx, dy)));
                    }
                }
            }

            draw_dusts(canvas, &mut dusts, step);
            Ok(surface.image_snapshot())
        } else {
            let mut surface = new_surface(img.dimensions());
            let canvas = surface.canvas();
            draw_dusts(canvas, &mut dusts, step);
            Ok(surface.image_snapshot())
        }
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 35,
            duration: 0.08,
        },
        None,
    )
}

register_meme! {
    "fade_away",
    fade_away,
    min_images = 1,
    max_images = 1,
    keywords = &["灰飞烟灭"],
    date_created = local_date(2024, 8, 20),
    date_modified = local_date(2025, 1, 14),
}
