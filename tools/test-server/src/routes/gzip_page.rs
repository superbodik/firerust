use std::io::Write;

use flate2::Compression;
use flate2::write::GzEncoder;

use crate::server::http_reply::HttpReply;

pub fn gzip_page() -> HttpReply {
    let html =
        "<html><head><title>Gzip</title></head><body><p>compressed with gzip</p></body></html>";
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    let _ = encoder.write_all(html.as_bytes());
    let bytes = encoder.finish().unwrap_or_default();
    HttpReply::new(200, "text/html; charset=utf-8", bytes).with_header("Content-Encoding", "gzip")
}
