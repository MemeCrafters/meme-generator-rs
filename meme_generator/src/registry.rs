use std::collections::HashMap;

use meme_generator_core::meme::Meme;

use crate::config::CONFIG;

struct MemeRegistry {
    memes: HashMap<String, Box<dyn Meme>>,
}

impl MemeRegistry {
    fn new() -> Self {
        Self {
            memes: HashMap::default(),
        }
    }
}

impl meme_generator_core::registry::MemeRegistry for MemeRegistry {
    fn register_meme(&mut self, name: &str, meme: Box<dyn Meme>) {
        self.memes.insert(name.to_string(), meme);
    }
}

pub fn load_memes() -> HashMap<String, Box<dyn Meme>> {
    let mut registry = MemeRegistry::new();
    if CONFIG.meme.load_builtin_memes {
        meme_generator_memes::register_memes(&mut registry);
    }
    registry.memes
}
