mod config;
mod memes;
mod registry;
mod search;
mod version;

pub mod resources;
pub mod tools;
pub use meme_generator_core::{
    config::{MEME_HOME, read_config_file},
    error, meme,
};
pub use memes::{get_meme, get_meme_keys, get_memes};
pub use search::search_memes;
pub use version::VERSION;
