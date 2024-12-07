use std::{fs::read, path::PathBuf};

use skia_safe::Image;

use crate::{decoder::decode_image, error::Error};

use chrono::{DateTime, Local, TimeZone};
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
    user_dirs.home_dir().join(".meme_generator")
}

pub fn local_date(year: i32, month: u32, day: u32) -> DateTime<Local> {
    Local.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()
}

pub fn load_image(path: &str) -> Result<Image, Error> {
    let image_path = meme_home().join("resources/images").join(path);
    decode_image(&read(image_path)?)
}
