use std::{collections::HashMap, sync::LazyLock};

use pinyin::{Pinyin, to_pinyin_vec};
use serde::{Deserialize, Serialize};

use meme_generator_core::meme::Meme;

use crate::registry::load_memes;

static LOADED_MEMES: LazyLock<HashMap<String, Box<dyn Meme>>> = LazyLock::new(|| load_memes());

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemeSortBy {
    Key,
    Keywords,
    KeywordsPinyin,
    DateCreated,
    DateModified,
}

pub fn get_meme(key: &str) -> Option<&'static Box<dyn Meme>> {
    LOADED_MEMES.get(key)
}

fn sort_memes(memes: &mut Vec<&Box<dyn Meme>>, sort_by: &MemeSortBy, sort_reverse: bool) {
    let keywords = |meme: &Box<dyn Meme>| meme.info().keywords.join("/");
    let keywords_pinyin =
        |meme: &Box<dyn Meme>| to_pinyin_vec(keywords(meme).as_str(), Pinyin::plain).join(" ");

    match sort_by {
        MemeSortBy::Key => memes.sort_by(|a, b| a.key().cmp(&b.key())),
        MemeSortBy::Keywords => memes.sort_by(|a, b| keywords(a).cmp(&keywords(b))),
        MemeSortBy::KeywordsPinyin => {
            memes.sort_by(|a, b| keywords_pinyin(a).cmp(&keywords_pinyin(b)))
        }
        MemeSortBy::DateCreated => {
            memes.sort_by(|a, b| a.info().date_created.cmp(&b.info().date_created))
        }
        MemeSortBy::DateModified => {
            memes.sort_by(|a, b| a.info().date_modified.cmp(&b.info().date_modified))
        }
    };
    if sort_reverse {
        memes.reverse();
    }
}

pub fn get_memes_sorted(sort_by: MemeSortBy, sort_reverse: bool) -> Vec<&'static Box<dyn Meme>> {
    let mut memes = LOADED_MEMES.values().into_iter().collect::<Vec<_>>();
    sort_memes(&mut memes, &sort_by, sort_reverse);
    memes
}

pub fn get_meme_keys_sorted(sort_by: MemeSortBy, sort_reverse: bool) -> Vec<String> {
    get_memes_sorted(sort_by, sort_reverse)
        .into_iter()
        .map(|meme| meme.key())
        .collect()
}

pub fn get_memes() -> Vec<&'static Box<dyn Meme>> {
    get_memes_sorted(MemeSortBy::Key, false)
}

pub fn get_meme_keys() -> Vec<String> {
    get_meme_keys_sorted(MemeSortBy::Key, false)
}
