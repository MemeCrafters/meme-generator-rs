use std::path::PathBuf;

use directories::UserDirs;
use skia_safe::{surfaces, FilterMode, ISize, MipmapMode, SamplingOptions, Surface};

pub fn new_surface(size: impl Into<ISize>) -> Surface {
    surfaces::raster_n32_premul(size).unwrap()
}

pub fn default_sampling_options() -> SamplingOptions {
    SamplingOptions::new(FilterMode::Linear, MipmapMode::Linear)
}

pub fn meme_home() -> PathBuf {
    let user_dirs = UserDirs::new().unwrap();
    user_dirs.home_dir().join(".meme-generator")
}
