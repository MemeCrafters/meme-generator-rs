use std::{fs::read, path::PathBuf};

use skia_safe::{scalar, Color4f, Data, Image, Paint, PaintJoin, PaintStyle};

use crate::{decoder::CodecExt, error::Error};

use chrono::{DateTime, Local, TimeZone};
use directories::UserDirs;
use skia_safe::{surfaces, Codec, FilterMode, ISize, MipmapMode, SamplingOptions, Surface};

pub fn new_surface(size: impl Into<ISize>) -> Surface {
    surfaces::raster_n32_premul(size).unwrap()
}

pub fn new_paint(color: impl AsRef<Color4f>) -> Paint {
    let mut paint = Paint::new(color, None);
    paint.set_anti_alias(true);
    paint
}

pub fn new_stroke_paint(color: impl AsRef<Color4f>, stroke_width: scalar) -> Paint {
    let mut paint = Paint::new(color, None);
    paint.set_anti_alias(true);
    paint.set_stroke_width(stroke_width);
    paint.set_style(PaintStyle::Stroke);
    paint.set_stroke_join(PaintJoin::Round);
    paint
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

pub fn load_image(path: impl Into<String>) -> Result<Image, Error> {
    let image_path = meme_home().join("resources/images").join(path.into());
    let data = Data::new_copy(&read(image_path)?);
    Codec::from_data(data)
        .ok_or(Error::ImageDecodeError(None))?
        .first_frame()
}
