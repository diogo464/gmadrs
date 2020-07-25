use gma;
use lzma_rs;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    CompressionError(lzma_rs::error::Error),
    GMAError(gma::Error),
    InvalidAddonJson(String),
    ToManyAddonTags,
    Custom(Box<dyn std::error::Error>),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}

impl From<lzma_rs::error::Error> for Error {
    fn from(e: lzma_rs::error::Error) -> Self {
        Self::CompressionError(e)
    }
}

impl From<gma::Error> for Error {
    fn from(e: gma::Error) -> Self {
        Self::GMAError(e)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
