use crate::server::http_reply::HttpReply;

pub const CYRILLIC_TEXT: &str = "Привет, мир";

pub fn cp1251_page() -> HttpReply {
    let html = format!(
        "<!DOCTYPE html><html><head><title>{CYRILLIC_TEXT}</title></head><body><p>{CYRILLIC_TEXT}</p></body></html>"
    );
    let (bytes, _, _) = encoding_rs::WINDOWS_1251.encode(&html);
    HttpReply::new(200, "text/html; charset=windows-1251", bytes.into_owned())
}
