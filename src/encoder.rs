use gif::{DisposalMethod, Encoder, Frame, Repeat};
use skia_safe::{EncodedImageFormat, Image};

use crate::error::{EncodeError, Error};
use crate::utils::get_image_pixels;

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
            let mut data = get_image_pixels(image)?;
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
