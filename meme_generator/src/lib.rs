pub mod config;
pub mod error;
pub mod manager;
pub mod meme;
pub mod resources;
pub(crate) mod tags;
pub mod utils;
pub mod version;

#[cfg(feature = "contrib")]
mod contrib;
mod memes;
