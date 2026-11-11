use std::path::PathBuf;

const OVERRIDE_VARIABLE: &str = "FIRERUST_DOWNLOADS";

pub fn downloads_folder() -> PathBuf {
    if let Some(custom) = std::env::var_os(OVERRIDE_VARIABLE).filter(|value| !value.is_empty()) {
        return PathBuf::from(custom);
    }
    if let Some(home) = std::env::var_os("USERPROFILE") {
        let folder = PathBuf::from(home).join("Downloads");
        if folder.is_dir() {
            return folder;
        }
    }
    std::env::temp_dir()
}
