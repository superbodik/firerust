use std::path::Path;

use crate::import::import_csv::import_csv;
use crate::import::password_import_error::PasswordImportError;
use crate::model::credential::Credential;

pub fn import_password_file(path: &Path) -> Result<Vec<Credential>, PasswordImportError> {
    if !path.is_file() {
        return Err(PasswordImportError::NotFound(path.display().to_string()));
    }
    let bytes =
        std::fs::read(path).map_err(|error| PasswordImportError::Unreadable(error.to_string()))?;
    let text = String::from_utf8_lossy(&bytes);
    let trimmed = text.trim_start_matches('\u{feff}');
    import_csv(trimmed).ok_or(PasswordImportError::UnknownFormat)
}
