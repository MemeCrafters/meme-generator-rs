use std::{collections::HashMap, sync::LazyLock};

use meme_generator_core::meme::Meme;

use crate::registry::load_memes;

static LOADED_MEMES: LazyLock<HashMap<String, Box<dyn Meme>>> = LazyLock::new(|| load_memes());

pub fn get_meme(key: &str) -> Option<&'static Box<dyn Meme>> {
    LOADED_MEMES.get(key)
}

pub fn get_memes() -> Vec<&'static Box<dyn Meme>> {
    let mut memes = LOADED_MEMES.values().into_iter().collect::<Vec<_>>();
    memes.sort_by_key(|meme| meme.key());
    memes
}

pub fn get_meme_keys() -> Vec<&'static str> {
    let mut keys = LOADED_MEMES
        .keys()
        .map(|key| key.as_str())
        .collect::<Vec<_>>();
    keys.sort();
    keys
}
