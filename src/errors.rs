use std::io;
use serde_bencode;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    BencodeError(serde_bencode::Error),
    ApiError(ApiError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_bencode::Error> for Error {
    fn from(err: serde_bencode::Error) -> Error {
        Error::BencodeError(err)
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
