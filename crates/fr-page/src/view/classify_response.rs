use fr_download::name::parse_content_disposition::parse_content_disposition;
use fr_net::load::response::Response;

use crate::view::looks_like_text::looks_like_text;
use crate::view::view_kind::ViewKind;

const DECODABLE_IMAGES: [&str; 4] = ["image/png", "image/jpeg", "image/gif", "image/webp"];
const TEXT_APPLICATIONS: [&str; 6] = [
    "application/json",
    "application/javascript",
    "application/x-javascript",
    "application/xml",
    "application/x-www-form-urlencoded",
    "application/ld+json",
];

pub fn classify_response(response: &Response) -> ViewKind {
    let is_attachment = response
        .content_disposition
        .as_deref()
        .is_some_and(|header| parse_content_disposition(header).is_attachment);
    if is_attachment {
        return ViewKind::Download;
    }
    let essence = response
        .content_type
        .as_deref()
        .map(|value| {
            value
                .split(';')
                .next()
                .unwrap_or("")
                .trim()
                .to_ascii_lowercase()
        })
        .unwrap_or_default();
    match essence.as_str() {
        "" => {
            if looks_like_text(&response.body) {
                ViewKind::Html
            } else {
                ViewKind::Download
            }
        }
        "text/html" | "application/xhtml+xml" => ViewKind::Html,
        "image/svg+xml" => ViewKind::Text,
        other if DECODABLE_IMAGES.contains(&other) => ViewKind::Image,
        other if other.starts_with("text/") => ViewKind::Text,
        other if TEXT_APPLICATIONS.contains(&other) => ViewKind::Text,
        other if other.ends_with("+json") || other.ends_with("+xml") => ViewKind::Text,
        _ => ViewKind::Download,
    }
}
