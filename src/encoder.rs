use gif::{DisposalMethod, Encoder, Frame, Repeat};
use skia_safe::{image::CachingHint, AlphaType, ColorType, EncodedImageFormat, Image, ImageInfo};

use crate::error::{EncodeError, Error};

pub fn encode_gif(images: &Vec<Image>, duration: f32) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new();
    let delay = (duration * 100.0) as u16;
    {
        let mut encoder = Encoder::new(
            &mut bytes,
            images[0].width() as u16,
            images[0].height() as u16,
            &[],
        )?;
        encoder.set_repeat(Repeat::Infinite)?;
        for image in images {
            let image_info = ImageInfo::new(
                image.dimensions(),
                ColorType::RGBA8888,
                AlphaType::Unpremul,
                None,
            );
            let row_bytes = image_info.min_row_bytes();
            let data_size = image_info.compute_min_byte_size();
            let mut data = vec![0u8; data_size];
            image.read_pixels(
                &image_info,
                &mut data,
                row_bytes,
                (0, 0),
                CachingHint::Allow,
            );
            let mut frame =
                Frame::from_rgba_speed(image.width() as u16, image.height() as u16, &mut data, 10);
            frame.delay = delay;
            frame.dispose = DisposalMethod::Background;
            encoder.write_frame(&frame)?;
        }
    }
    Ok(bytes)
}

fn encode_image(
    image: &Image,
    format: EncodedImageFormat,
    quality: impl Into<Option<u32>>,
) -> Result<Vec<u8>, Error> {
    let data = image
        .encode(None, format, quality)
        .ok_or(EncodeError::SkiaEncodeError)?;
    Ok(data.as_bytes().to_vec())
}

pub fn encode_png(image: &Image) -> Result<Vec<u8>, Error> {
    encode_image(image, EncodedImageFormat::PNG, None)
}

pub fn encode_jpg(image: &Image) -> Result<Vec<u8>, Error> {
    encode_image(image, EncodedImageFormat::JPEG, 90)
}
