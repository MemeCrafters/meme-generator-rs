use skia_safe::codec;
use std::io;

#[derive(Debug)]
pub enum EncodeError {
    GifEncodeError(gif::EncodingError),
    SkiaEncodeError,
}

#[derive(Debug)]
pub enum Error {
    ImageDecodeError(Option<codec::Result>),
    ImageEncodeError(EncodeError),
    IOError(io::Error),
}

impl From<codec::Result> for Error {
    fn from(err: codec::Result) -> Self {
        Error::ImageDecodeError(Some(err))
    }
}

impl From<gif::EncodingError> for Error {
    fn from(err: gif::EncodingError) -> Self {
        Error::ImageEncodeError(EncodeError::GifEncodeError(err))
    }
}

impl From<EncodeError> for Error {
    fn from(err: EncodeError) -> Self {
        Error::ImageEncodeError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IOError(err)
    }
}
