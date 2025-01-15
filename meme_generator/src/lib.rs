mod config;
mod registry;
mod search;
mod version;

pub mod resources;
pub use meme_generator_core::{
    config::{read_config_file, MEME_HOME},
    error, meme,
};
pub use registry::load_memes;
pub use search::search_memes;
pub use version::VERSION;
