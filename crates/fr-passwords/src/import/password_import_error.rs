use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum PasswordImportError {
    NotFound(String),
    Unreadable(String),
    UnknownFormat,
}

impl fmt::Display for PasswordImportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(path) => write!(formatter, "{path} was not found"),
            Self::Unreadable(reason) => write!(formatter, "cannot read the file: {reason}"),
            Self::UnknownFormat => formatter.write_str(
                "the file is not a CSV list of passwords (it needs url, username and password columns)",
            ),
        }
    }
}

impl Error for PasswordImportError {}
