use std::fmt;
use std::io;

#[derive(Debug)]
pub enum DownloadError {
    CreateFolder(io::Error),
    WriteFile(io::Error),
    NoFreeName,
}

impl fmt::Display for DownloadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateFolder(error) => write!(formatter, "cannot create the folder: {error}"),
            Self::WriteFile(error) => write!(formatter, "cannot write the file: {error}"),
            Self::NoFreeName => write!(formatter, "no free file name in the folder"),
        }
    }
}

impl std::error::Error for DownloadError {}
