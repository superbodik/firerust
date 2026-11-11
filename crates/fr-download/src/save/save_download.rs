use std::fs::{self, OpenOptions};
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};

use fr_net::load::response::Response;

use crate::name::file_name_for::file_name_for;
use crate::save::candidate_path::{MAX_ATTEMPTS, candidate_path};
use crate::save::download_error::DownloadError;

pub fn save_download(response: &Response, folder: &Path) -> Result<PathBuf, DownloadError> {
    fs::create_dir_all(folder).map_err(DownloadError::CreateFolder)?;
    let name = file_name_for(response);
    for attempt in 0..=MAX_ATTEMPTS {
        let path = candidate_path(folder, &name, attempt);
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(mut file) => {
                file.write_all(&response.body)
                    .map_err(DownloadError::WriteFile)?;
                return Ok(path);
            }
            Err(error) if error.kind() == ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(DownloadError::WriteFile(error)),
        }
    }
    Err(DownloadError::NoFreeName)
}
