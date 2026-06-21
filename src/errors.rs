use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io {
        #[allow(dead_code)]
        source: io::Error,
    },
    Eyre {
        #[allow(dead_code)]
        source: color_eyre::eyre::Error,
    },
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Io { source: value }
    }
}

impl From<color_eyre::eyre::Error> for Error {
    fn from(value: color_eyre::eyre::Error) -> Self {
        Error::Eyre { source: value }
    }
}
