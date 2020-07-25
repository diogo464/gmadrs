use gma;
use lzma_rs;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    CompressionError(lzma_rs::error::Error),
    GMAError(gma::Error),
    InvalidAddonJson(String),
    ToManyAddonTags,
    FileNotInArchive(String),
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

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(e) => e.fmt(f),
            Self::CompressionError(e) => write!(f, "Compression error : {:?}", e),
            Self::GMAError(e) => e.fmt(f),
            Self::InvalidAddonJson(m) => write!(f, "{}", m),
            Self::ToManyAddonTags => write!(f, "Only 2 tags are allowed on the addon"),
            Self::FileNotInArchive(filename) => {
                write!(f, "The file '{}' was not in the archive", filename)
            }
            Self::Custom(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T, E = Error> = std::result::Result<T, E>;
