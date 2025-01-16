use rand::seq::SliceRandom;
use skia_safe::{Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    encoder::make_png_or_gif,
    image::{Fit, ImageExt},
    tools::{load_image, local_date, new_surface},
};

use crate::{register_meme, tags::MemeTags, union_tags};

#[derive(MemeOptions)]
struct Character {
    /// 角色名
    #[option(short, long, choices = ["arona", "plana"])]
    character: Option<String>,
}

fn keep_your_money(
    images: Vec<InputImage>,
    _: Vec<String>,
    options: Character,
) -> Result<Vec<u8>, Error> {
    let character = options.character.as_deref().unwrap_or({
        let mut rng = rand::thread_rng();
        ["arona", "plana"].choose(&mut rng).unwrap()
    });
    let frame = load_image(format!("keep_your_money/{character}.png"))?;

    let func = |images: Vec<Image>| {
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        let image = images[0].resize_fit((500, 640), Fit::Contain);
        canvas.draw_image(&image, (0, 440), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "keep_your_money",
    keep_your_money,
    min_images = 1,
    max_images = 1,
    keywords = &["压岁钱不要交给"],
    tags = union_tags!(MemeTags::arona(), MemeTags::plana()),
    date_created = local_date(2024, 12, 29),
    date_modified = local_date(2024, 12, 31),
);
