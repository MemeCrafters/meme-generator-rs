use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, Mutex},
};

use crate::meme::Meme;

pub(crate) struct MemeRegistry {
    memes: HashMap<String, Arc<dyn Meme>>,
}

impl MemeRegistry {
    pub fn new() -> Self {
        Self {
            memes: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: String, meme: Arc<dyn Meme>) {
        if self.memes.contains_key(&name) {
            panic!("Meme `{name}` is already registered");
        }
        self.memes.insert(name, meme);
    }
}

pub(crate) static MEME_REGISTRY: LazyLock<Mutex<MemeRegistry>> =
    LazyLock::new(|| Mutex::new(MemeRegistry::new()));

macro_rules! register_meme {
    ($key:expr, $function:expr, $($field:ident = $value:expr),* $(,)?) => {
        #[ctor::ctor]
        fn register_plugin() {
            let mut registry = $crate::manager::MEME_REGISTRY.lock().unwrap();
            let meme = std::sync::Arc::new(crate::meme::MemeBuilder {
                key: $key.to_string(),
                function: $function,
                $(
                    $field: crate::meme::meme_setters::$field($value),
                )*
                ..Default::default()
            }) as std::sync::Arc<dyn crate::meme::Meme>;
            registry.register($key.to_string(), meme);
        }
    };
}

pub(crate) use register_meme;

pub fn get_meme(key: &str) -> Option<Arc<dyn Meme>> {
    let registry = MEME_REGISTRY.lock().unwrap();
    registry.memes.get(key).cloned()
}

pub fn get_memes() -> Vec<Arc<dyn Meme>> {
    let registry = MEME_REGISTRY.lock().unwrap();
    let mut memes = registry.memes.values().cloned().collect::<Vec<_>>();
    memes.sort_by_key(|meme| meme.key());
    memes
}

pub fn get_meme_keys() -> Vec<String> {
    let registry = MEME_REGISTRY.lock().unwrap();
    let mut keys = registry.memes.keys().cloned().collect::<Vec<_>>();
    keys.sort();
    keys
}
