use skia_safe::image::CachingHint;
use skia_safe::{surfaces, AlphaType, ColorType, ISize, Image, ImageInfo, Surface};

use crate::error::Error;

pub fn get_image_pixels(image: &Image) -> Result<Vec<u8>, Error> {
    let image_info = ImageInfo::new(
        (image.width() as i32, image.height() as i32),
        ColorType::RGBA8888,
        AlphaType::Unpremul,
        None,
    );
    let row_bytes = image_info.min_row_bytes();
    let data_size = image_info.compute_min_byte_size();
    let mut data = vec![0u8; data_size];
    let result = image.read_pixels(
        &image_info,
        &mut data,
        row_bytes,
        (0, 0),
        CachingHint::Allow,
    );
    if result == true {
        Ok(data)
    } else {
        Err(Error::InternalError)
    }
}

pub fn new_surface(size: impl Into<ISize>) -> Result<Surface, Error> {
    Ok(surfaces::raster_n32_premul(size).ok_or(Error::InternalError)?)
}
