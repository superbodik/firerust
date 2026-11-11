use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadRecord {
    pub name: String,
    pub path: PathBuf,
    pub bytes: usize,
}
