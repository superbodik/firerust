use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ImportError {
    NotFound(String),
    Unreadable(String),
    UnknownFormat,
}

impl fmt::Display for ImportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(what) => write!(formatter, "{what} was not found"),
            Self::Unreadable(reason) => write!(formatter, "cannot read the file: {reason}"),
            Self::UnknownFormat => formatter.write_str("the file is not a list of bookmarks"),
        }
    }
}

impl Error for ImportError {}
