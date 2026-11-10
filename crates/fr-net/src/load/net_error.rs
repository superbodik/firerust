use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum NetError {
    UnsupportedScheme(String),
    InvalidFilePath,
    FileRead(io::Error),
    Status(u16),
    Http(String),
}

impl fmt::Display for NetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedScheme(scheme) => {
                write!(formatter, "cannot load scheme: {scheme}")
            }
            Self::InvalidFilePath => formatter.write_str("file address has no valid path"),
            Self::FileRead(error) => write!(formatter, "cannot read file: {error}"),
            Self::Status(code) => write!(formatter, "server answered with status {code}"),
            Self::Http(message) => write!(formatter, "request failed: {message}"),
        }
    }
}

impl Error for NetError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::FileRead(error) => Some(error),
            _ => None,
        }
    }
}
