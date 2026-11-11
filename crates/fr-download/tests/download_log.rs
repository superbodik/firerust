use std::path::Path;
use std::sync::Mutex;

use fr_download::record::download_log::{clear_downloads, recent_downloads, remember_download};

static SERIAL: Mutex<()> = Mutex::new(());

#[test]
fn downloads_come_back_newest_first() {
    let _guard = SERIAL
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    clear_downloads();

    remember_download(Path::new("C:\\Downloads\\first.zip"), 10);
    remember_download(Path::new("C:\\Downloads\\second.pdf"), 2048);

    let recent = recent_downloads();
    assert_eq!(recent.len(), 2);
    assert_eq!(recent[0].name, "second.pdf");
    assert_eq!(recent[0].bytes, 2048);
    assert_eq!(recent[1].name, "first.zip");
    clear_downloads();
}

#[test]
fn the_log_keeps_only_the_latest_two_hundred() {
    let _guard = SERIAL
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    clear_downloads();

    for number in 0..250 {
        remember_download(
            Path::new(&format!("C:\\Downloads\\file{number}.bin")),
            number,
        );
    }

    let recent = recent_downloads();
    assert_eq!(recent.len(), 200);
    assert_eq!(recent[0].name, "file249.bin");
    assert_eq!(recent[199].name, "file50.bin");
    clear_downloads();
}
