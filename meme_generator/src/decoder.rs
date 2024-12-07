use skia_safe::{codec, AlphaType, Codec, ColorType, Data, ISize, Image, ImageInfo};

use crate::error::Error;

pub struct Decoder<'a> {
    codec: Codec<'a>,
}

impl<'a> Decoder<'a> {
    pub fn from_data(data: &Vec<u8>) -> Result<Decoder<'static>, Error> {
        let data = Data::new_copy(data);
        let codec = Codec::from_data(&data).ok_or(Error::ImageDecodeError(None))?;
        Ok(Decoder { codec })
    }

    pub fn info(&self) -> ImageInfo {
        self.codec.info()
    }

    pub fn dimensions(&self) -> ISize {
        self.codec.dimensions()
    }

    pub fn is_multi_frame(&mut self) -> bool {
        self.get_frame_count() > 1
    }

    pub fn get_frame_count(&mut self) -> usize {
        self.codec.get_frame_count()
    }

    pub fn get_frame_info(&mut self, index: usize) -> Result<codec::FrameInfo, Error> {
        self.codec
            .get_frame_info(index)
            .ok_or(Error::ImageDecodeError(None))
    }

    pub fn get_average_duration(&mut self) -> Result<f32, Error> {
        let count = self.get_frame_count();
        let mut total_duration = 0.0;
        for i in 0..count {
            let frame_info = self.get_frame_info(i)?;
            total_duration += frame_info.duration as f32 / 1000.0;
        }
        Ok(total_duration / count as f32)
    }

    pub fn first_image(&mut self) -> Result<Image, Error> {
        self.get_image(0)
    }

    pub fn get_image(&mut self, index: usize) -> Result<Image, Error> {
        let image_info = ImageInfo::new(
            self.codec.dimensions(),
            ColorType::RGBA8888,
            AlphaType::Unpremul,
            None,
        );
        let options = codec::Options {
            zero_initialized: codec::ZeroInitialized::Yes,
            subset: None,
            frame_index: index,
            prior_frame: if index == 0 { None } else { Some(index - 1) },
        };
        Ok(self.codec.get_image(image_info, &options)?)
    }
}

pub fn decode_image(data: &Vec<u8>) -> Result<Image, Error> {
    Decoder::from_data(data)?.first_image()
}
