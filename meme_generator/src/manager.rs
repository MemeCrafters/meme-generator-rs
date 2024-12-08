use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;

use crate::meme::Meme;

pub struct MemeRegistry {
    memes: HashMap<String, Arc<dyn Meme>>,
}

impl MemeRegistry {
    pub fn new() -> Self {
        Self {
            memes: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: String, meme: Arc<dyn Meme>) {
        self.memes.insert(name, meme);
    }
}

lazy_static! {
    pub static ref MEME_REGISTRY: Mutex<MemeRegistry> = Mutex::new(MemeRegistry::new());
}

#[macro_export]
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

pub fn get_meme(key: &str) -> Option<Arc<dyn Meme>> {
    let registry = MEME_REGISTRY.lock().unwrap();
    registry.memes.get(key).cloned()
}

pub fn get_memes() -> Vec<Arc<dyn Meme>> {
    let registry = MEME_REGISTRY.lock().unwrap();
    registry.memes.values().cloned().collect()
}

pub fn get_meme_keys() -> Vec<String> {
    let registry = MEME_REGISTRY.lock().unwrap();
    registry.memes.keys().cloned().collect()
}
