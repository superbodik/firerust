use fr_net::address::percent_decode::percent_decode;

use crate::name::content_disposition::ContentDisposition;

pub fn parse_content_disposition(header: &str) -> ContentDisposition {
    let mut parts = header.split(';');
    let kind = parts.next().unwrap_or("").trim().to_ascii_lowercase();
    let mut plain: Option<String> = None;
    let mut extended: Option<String> = None;
    for part in parts {
        let Some((key, value)) = part.split_once('=') else {
            continue;
        };
        let key = key.trim().to_ascii_lowercase();
        let value = value.trim();
        if key == "filename*" {
            extended = decode_extended(value);
        } else if key == "filename" {
            plain = Some(unquote(value));
        }
    }
    ContentDisposition {
        is_attachment: kind == "attachment",
        file_name: extended.or(plain).filter(|name| !name.is_empty()),
    }
}

fn unquote(value: &str) -> String {
    value
        .strip_prefix('"')
        .and_then(|inner| inner.strip_suffix('"'))
        .unwrap_or(value)
        .replace("\\\"", "\"")
}

fn decode_extended(value: &str) -> Option<String> {
    let (charset, rest) = value.split_once('\u{27}')?;
    let (_language, encoded) = rest.split_once('\u{27}')?;
    if !charset.eq_ignore_ascii_case("utf-8") {
        return None;
    }
    Some(percent_decode(encoded))
}
