use std::path::Path;

use crate::import::import_chrome::import_chrome;
use crate::import::import_error::ImportError;
use crate::import::import_html::import_html;
use crate::model::bookmark::Bookmark;

pub fn import_from_file(path: &Path) -> Result<Vec<Bookmark>, ImportError> {
    if !path.is_file() {
        return Err(ImportError::NotFound(path.display().to_string()));
    }
    let bytes = std::fs::read(path).map_err(|error| ImportError::Unreadable(error.to_string()))?;
    let text = String::from_utf8_lossy(&bytes);
    let trimmed = text.trim_start_matches('\u{feff}').trim_start();
    if trimmed.starts_with('{') {
        return import_chrome(trimmed).ok_or(ImportError::UnknownFormat);
    }
    let found = import_html(trimmed);
    if found.is_empty() {
        return Err(ImportError::UnknownFormat);
    }
    Ok(found)
}
