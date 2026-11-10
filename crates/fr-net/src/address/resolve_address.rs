use std::path::{Path, absolute};

use url::Url;

use crate::address::address_error::AddressError;

pub fn resolve_address(input: &str, base_dir: &Path) -> Result<Url, AddressError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(AddressError::Empty);
    }
    if let Some(url) = existing_file_url(trimmed, base_dir) {
        return Ok(url);
    }
    let candidate = if trimmed.contains("://") {
        trimmed.to_owned()
    } else {
        format!("https://{trimmed}")
    };
    let url = Url::parse(&candidate).map_err(|_| AddressError::Invalid(trimmed.to_owned()))?;
    check_scheme(url)
}

fn existing_file_url(input: &str, base_dir: &Path) -> Option<Url> {
    let path = absolute(base_dir.join(input)).ok()?;
    if !path.is_file() {
        return None;
    }
    Url::from_file_path(path).ok()
}

fn check_scheme(url: Url) -> Result<Url, AddressError> {
    match url.scheme() {
        "http" | "https" | "file" => Ok(url),
        other => Err(AddressError::UnsupportedScheme(other.to_owned())),
    }
}
