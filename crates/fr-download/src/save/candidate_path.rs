use std::path::{Path, PathBuf};

pub const MAX_ATTEMPTS: u32 = 999;

pub fn candidate_path(folder: &Path, name: &str, attempt: u32) -> PathBuf {
    if attempt == 0 {
        return folder.join(name);
    }
    let path = Path::new(name);
    let stem = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or(name);
    match path.extension().and_then(|extension| extension.to_str()) {
        Some(extension) => folder.join(format!("{stem} ({attempt}).{extension}")),
        None => folder.join(format!("{stem} ({attempt})")),
    }
}
