use std::io;

use bencode;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    DecoderError(bencode::DecoderError),
    StreamingError(bencode::streaming::Error),
    ApiError(ApiError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<bencode::DecoderError> for Error {
    fn from(err: bencode::DecoderError) -> Error {
        Error::DecoderError(err)
    }
}

impl From<bencode::streaming::Error> for Error {
    fn from(err: bencode::streaming::Error) -> Error {
        Error::StreamingError(err)
    }
}

impl From<ApiError> for Error {
    fn from(err: ApiError) -> Error {
        Error::ApiError(err)
    }
}

#[derive(Debug)]
pub struct ApiError {
    pub error: String,
}

impl ApiError {
    pub fn new(err: String) -> ApiError {
        ApiError {
            error: err,
        }
    }
}
