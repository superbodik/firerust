pub fn write_text(text: &str) -> bool {
    let Ok(mut clipboard) = arboard::Clipboard::new() else {
        return false;
    };
    clipboard.set_text(text.to_owned()).is_ok()
}
