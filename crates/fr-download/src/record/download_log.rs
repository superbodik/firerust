use std::path::Path;
use std::sync::Mutex;

use crate::record::download_record::DownloadRecord;

const CAPACITY: usize = 200;

static RECORDS: Mutex<Vec<DownloadRecord>> = Mutex::new(Vec::new());

pub fn remember_download(path: &Path, bytes: usize) {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("file")
        .to_owned();
    let Ok(mut records) = RECORDS.lock() else {
        return;
    };
    if records.len() >= CAPACITY {
        records.remove(0);
    }
    records.push(DownloadRecord {
        name,
        path: path.to_path_buf(),
        bytes,
    });
}

pub fn recent_downloads() -> Vec<DownloadRecord> {
    RECORDS
        .lock()
        .map(|records| records.iter().rev().cloned().collect())
        .unwrap_or_default()
}

pub fn clear_downloads() {
    if let Ok(mut records) = RECORDS.lock() {
        records.clear();
    }
}
