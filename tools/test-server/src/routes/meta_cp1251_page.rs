use crate::routes::cp1251_page::CYRILLIC_TEXT;
use crate::server::http_reply::HttpReply;

pub fn meta_cp1251_page() -> HttpReply {
    let html = format!(
        "<!DOCTYPE html><html><head><meta charset=\"windows-1251\"><title>{CYRILLIC_TEXT}</title></head><body><p>{CYRILLIC_TEXT}</p></body></html>"
    );
    let (bytes, _, _) = encoding_rs::WINDOWS_1251.encode(&html);
    HttpReply::new(200, "text/html", bytes.into_owned())
}
