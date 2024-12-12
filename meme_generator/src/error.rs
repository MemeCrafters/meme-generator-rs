use std::{error, fmt, io};

use skia_safe::codec;

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
    DeserializeError(serde_json::Error),
    ImageNumberMismatch(u8, u8, u8),
    TextNumberMismatch(u8, u8, u8),
    TextOverLength(String),
    MemeFeedback(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ImageDecodeError(Some(err)) => write!(f, "Failed to decode image: {err:?}"),
            Error::ImageDecodeError(None) => write!(f, "Failed to decode image"),
            Error::ImageEncodeError(EncodeError::GifEncodeError(err)) => {
                write!(f, "Failed to encode image as GIF: {err}")
            }
            Error::ImageEncodeError(EncodeError::SkiaEncodeError) => {
                write!(f, "Failed to encode image")
            }
            Error::IOError(err) => write!(f, "IO error: {err}"),
            Error::DeserializeError(err) => write!(f, "Failed to deserialize: {err}"),
            Error::ImageNumberMismatch(min, max, actual) => write!(
                f,
                "Image number mismatch: expected between {min} and {max}, got {actual}",
            ),
            Error::TextNumberMismatch(min, max, actual) => write!(
                f,
                "Text number mismatch: expected between {min} and {max}, got {actual}",
            ),
            Error::TextOverLength(text) => write!(f, "Text is too long: {text}"),
            Error::MemeFeedback(feedback) => write!(f, "{feedback}"),
        }
    }
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

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::DeserializeError(err)
    }
}

impl error::Error for Error {}
