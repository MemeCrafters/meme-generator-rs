use rand::seq::IndexedRandom;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    encoder::GifEncoder,
    image::ImageExt,
    shortcut,
    tools::{load_image, local_date},
};

use crate::{register_meme, tags::MemeTags, union_tags};

#[derive(MemeOptions)]
struct Character {
    /// 角色名
    #[option(short, long, choices = ["hutao", "keqing", "klee", "nilou", "yae_miko", "zhongli"])]
    character: Option<String>,
}

fn genshin_eat(
    images: Vec<InputImage>,
    _: Vec<String>,
    options: Character,
) -> Result<Vec<u8>, Error> {
    let character = options.character.as_deref().unwrap_or({
        let mut rng = rand::rng();
        ["hutao", "keqing", "klee", "nilou", "yae_miko", "zhongli"]
            .choose(&mut rng)
            .unwrap()
    });

    let locs = [(106, 245), (115, 224), (116, 205), (115, 198), (120, 217)];
    let image = images[0].image.circle().resize_exact((44, 44));

    let mut encoder = GifEncoder::new();
    for i in 0..16 {
        let frame = load_image(format!("genshin_eat/{character}/{:02}.png", i))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        if (4..9).contains(&i) {
            let image = if i == 8 {
                &image.resize_exact((44, 33))
            } else {
                &image
            };
            canvas.draw_image(&image, locs[i - 4], None);
        }
        encoder.add_frame(surface.image_snapshot(), 0.08)?;
    }
    Ok(encoder.finish()?)
}

register_meme!(
    "genshin_eat",
    genshin_eat,
    min_images = 1,
    max_images = 1,
    keywords = &["原神吃"],
    shortcuts = &[
        shortcut!("胡桃吃", options = &[("character", "hutao")]),
        shortcut!("刻晴吃", options = &[("character", "keqing")]),
        shortcut!("可莉吃", options = &[("character", "klee")]),
        shortcut!("妮露吃", options = &[("character", "nilou")]),
        shortcut!(
            r"(?:八重神子|神子|八重)吃",
            options = &[("character", "yae_miko")],
            humanized = "八重神子吃"
        ),
        shortcut!("钟离吃", options = &[("character", "zhongli")]),
    ],
    tags = union_tags!(
        MemeTags::hutao(),
        MemeTags::keqing(),
        MemeTags::klee(),
        MemeTags::nilou(),
        MemeTags::yae_miko(),
        MemeTags::zhongli(),
    ),
    date_created = local_date(2024, 8, 6),
    date_modified = local_date(2024, 8, 10),
);
