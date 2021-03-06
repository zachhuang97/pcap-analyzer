use nom::error::ErrorKind;
use nom::Err;
use pcap_parser::PcapError;
use std::convert::From;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Internal parser error {0:?}")]
    Nom(ErrorKind),
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    #[error("Pcap parser error {0:?}")]
    Pcap(#[from] PcapError),
    #[error("Generic error {0}")]
    Generic(&'static str),
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Self {
        Error::Generic(s)
    }
}

impl From<ErrorKind> for Error {
    fn from(e: ErrorKind) -> Self {
        Error::Nom(e)
    }
}

impl From<Err<PcapError>> for Error {
    fn from(err: Err<PcapError>) -> Self {
        match err {
            Err::Incomplete(_) => Error::Pcap(PcapError::Incomplete),
            Err::Error(e) | Err::Failure(e) => Error::Pcap(e),
        }
    }
}
