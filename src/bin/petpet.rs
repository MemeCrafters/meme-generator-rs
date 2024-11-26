use std::fs::File;
use std::io::Write;

use meme_generator::{
    canvas::{CanvasExt, Fit},
    decoder::load_image,
    encoder::encode_gif,
    error::Error,
    utils::new_surface,
};

use skia_safe::{Image, Rect};

fn petpet(images: &Vec<Image>, _: &Vec<String>) -> Result<Vec<u8>, Error> {
    let image = &images[0];
    let locs = [
        (14, 20, 98, 98),
        (12, 33, 101, 85),
        (8, 40, 110, 76),
        (10, 33, 102, 84),
        (12, 20, 98, 98),
    ];

    let mut frames: Vec<Image> = Vec::new();
    for i in 0..5 {
        let hand = load_image(format!("data/petpet/{}.png", i))?;
        let mut surface = new_surface(hand.dimensions())?;
        let canvas = surface.canvas();
        let (x, y, w, h) = locs[i];
        canvas.draw_image_rect_fit(
            &image,
            Rect::from_xywh(x as f32, y as f32, w as f32, h as f32),
            Fit::Fill,
        );
        canvas.draw_image(&hand, (0, 0), None);
        let frame = surface.image_snapshot();
        frames.push(frame);
    }
    encode_gif(&frames, 0.06)
}

fn main() {
    let avatar = load_image("data/avatar.jpg").unwrap();
    let result = petpet(&vec![avatar], &vec![]).unwrap();
    let mut file = File::create("result.gif").unwrap();
    file.write_all(result.as_slice()).unwrap();
}
