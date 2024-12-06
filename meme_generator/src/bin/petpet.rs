use std::fs::File;
use std::io::Write;

use meme_generator::{
    decoder::load_image,
    encoder::encode_gif,
    error::Error,
    image::ImageExt,
    meme::{IntoMemeOptions, MemeOption, MemeOptions, ParserFlags},
    utils::new_surface,
};

use skia_safe::Image;

#[derive(Debug, Clone, MemeOptions)]
struct Options {
    /// 是否将图片变为圆形
    #[option(short, long, short_aliases = ['圆'])]
    circle: bool,
}

fn petpet(images: &Vec<Image>, texts: &Vec<String>, options: &Options) -> Result<Vec<u8>, Error> {
    let image = &images[0];
    let mut image = image.square();
    if options.circle {
        image = image.circle();
    }
    let locs = [
        (14, 20, 98, 98),
        (12, 33, 101, 85),
        (8, 40, 110, 76),
        (10, 33, 102, 84),
        (12, 20, 98, 98),
    ];

    let mut frames: Vec<Image> = Vec::new();
    for i in 0..5 {
        let hand = load_image(format!("../resources/images/petpet/{}.png", i))?;
        let mut surface = new_surface(hand.dimensions());
        let canvas = surface.canvas();
        let (x, y, w, h) = locs[i];
        let image = image.resize_exact((w, h));
        canvas.draw_image(&image, (x, y), None);
        canvas.draw_image(&hand, (0, 0), None);
        let frame = surface.image_snapshot();
        frames.push(frame);
    }
    encode_gif(&frames, 0.06)
}

fn main() {
    let avatar = load_image("../avatar.jpg").unwrap();
    let result = petpet(&vec![avatar], &vec![], &Options { circle: true }).unwrap();
    let mut file = File::create("result.gif").unwrap();
    file.write_all(result.as_slice()).unwrap();
}
