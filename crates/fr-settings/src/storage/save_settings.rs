use std::io;
use std::path::Path;

use crate::model::settings::Settings;
use crate::storage::encode_settings::encode_settings;

pub fn save_settings(path: &Path, settings: &Settings) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let temporary = path.with_extension("json.tmp");
    std::fs::write(&temporary, encode_settings(settings))?;
    std::fs::rename(&temporary, path)
}
