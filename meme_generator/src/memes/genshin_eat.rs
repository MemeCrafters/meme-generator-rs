use rand::Rng;
use skia_safe::Image;

use crate::{
    error::Error,
    manager::register_meme,
    meme::{shortcut, DecodedImage, MemeOptions},
    tags::{union_tags, MemeTags},
    utils::{
        encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
        image::ImageExt,
        load_image, local_date,
    },
};

#[derive(MemeOptions)]
struct Character {
    /// 角色名
    #[option(short, long, default = "random", choices = ["random", "hutao", "keqing", "klee", "nilou", "yae_miko", "zhongli"])]
    character: String,
}

fn genshin_eat(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    options: &Character,
) -> Result<Vec<u8>, Error> {
    let character = if options.character.as_str() == "random" {
        let mut rng = rand::thread_rng();
        let characters = ["hutao", "keqing", "klee", "nilou", "yae_miko", "zhongli"];
        characters[rng.gen_range(0..characters.len())]
    } else {
        &options.character
    };

    let locs = [(106, 245), (115, 224), (116, 205), (115, 198), (120, 217)];

    let func = |i: usize, images: &Vec<Image>| {
        let frame = load_image(format!("genshin_eat/{character}/{:02}.png", i))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        if (4..9).contains(&i) {
            let mut image = images[0].circle().resize_exact((44, 44));
            if i == 8 {
                image = image.resize_exact((44, 33));
            }
            canvas.draw_image(&image, locs[i - 4], None);
        }
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 16,
            duration: 0.08,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "genshin_eat",
    genshin_eat,
    min_images = 1,
    max_images = 1,
    keywords = &["原神吃"],
    shortcuts = &[
        shortcut!("胡桃吃", parser_args = &["--character", "hutao"]),
        shortcut!("刻晴吃", parser_args = &["--character", "keqing"]),
        shortcut!("可莉吃", parser_args = &["--character", "klee"]),
        shortcut!("妮露吃", parser_args = &["--character", "nilou"]),
        shortcut!(
            r"(?:八重神子|神子|八重)吃",
            parser_args = &["--character", "yae_miko"],
            humanized = "八重神子吃"
        ),
        shortcut!("钟离吃", parser_args = &["--character", "zhongli"]),
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
