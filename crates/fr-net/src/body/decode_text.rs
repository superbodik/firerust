use encoding_rs::{Encoding, UTF_8, WINDOWS_1252};

use crate::body::charset_from_header::charset_from_header;
use crate::body::charset_from_markup::charset_from_markup;

pub fn decode_text(body: &[u8], content_type: Option<&str>) -> String {
    if let Some((encoding, _)) = Encoding::for_bom(body) {
        return encoding.decode(body).0.into_owned();
    }
    if let Some(encoding) = charset_from_header(content_type) {
        return encoding.decode_without_bom_handling(body).0.into_owned();
    }
    if let Some(encoding) = charset_from_markup(body) {
        return encoding.decode_without_bom_handling(body).0.into_owned();
    }
    match std::str::from_utf8(body) {
        Ok(text) => text.to_owned(),
        Err(_) => WINDOWS_1252
            .decode_without_bom_handling(body)
            .0
            .into_owned(),
    }
}

pub fn decode_utf8_lossy(body: &[u8]) -> String {
    UTF_8.decode_without_bom_handling(body).0.into_owned()
}
