use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;

use crate::meme::MemeTrait;

pub struct MemeRegistry {
    memes: HashMap<String, Box<dyn MemeTrait>>,
}

impl MemeRegistry {
    pub fn new() -> Self {
        Self {
            memes: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: String, meme: Box<dyn MemeTrait>) {
        self.memes.insert(name, meme);
    }

    pub fn get(&self, key: &str) -> Option<&Box<dyn MemeTrait>> {
        self.memes.get(key)
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
            let mut registry = $crate::registry::MEME_REGISTRY.lock().unwrap();
            let meme = Box::new(crate::meme::Meme {
                key: $key.to_string(),
                function: $function,
                $(
                    $field: crate::meme::meme_setters::$field($value),
                )*
                ..Default::default()
            }) as Box<dyn crate::meme::MemeTrait>;
            registry.register($key.to_string(), meme);
        }
    };
}
