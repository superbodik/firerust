use std::path::Path;

use crate::model::settings::Settings;
use crate::storage::decode_settings::decode_settings;

pub fn load_settings(path: &Path) -> Settings {
    let Ok(text) = std::fs::read_to_string(path) else {
        return Settings::default();
    };
    decode_settings(&text).unwrap_or_else(|| {
        eprintln!("cannot read settings from {}", path.display());
        Settings::default()
    })
}
