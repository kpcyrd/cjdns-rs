use std::io;

use bencode;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    DecoderError(bencode::DecoderError),
    StreamingError(bencode::streaming::Error),
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
