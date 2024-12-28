use std::{error, fmt, io};

use skia_safe::codec;

#[derive(Debug)]
pub enum Error {
    ImageDecodeError(String),
    ImageEncodeError(String),
    IOError(String),
    DeserializeError(String),
    ImageNumberMismatch(u8, u8, u8),
    TextNumberMismatch(u8, u8, u8),
    TextOverLength(String),
    MemeFeedback(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ImageDecodeError(err) => write!(f, "Failed to decode image: {err}"),
            Error::ImageEncodeError(err) => write!(f, "Failed to encode image: {err}"),
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
        Error::ImageDecodeError(format!("{:?}", err))
    }
}

impl From<gif::EncodingError> for Error {
    fn from(err: gif::EncodingError) -> Self {
        Error::ImageEncodeError(err.to_string())
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IOError(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::DeserializeError(err.to_string())
    }
}

impl error::Error for Error {}
