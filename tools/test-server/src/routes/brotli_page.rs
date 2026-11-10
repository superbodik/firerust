use std::io::Write;

use brotli::CompressorWriter;

use crate::server::http_reply::HttpReply;

pub fn brotli_page() -> HttpReply {
    let html =
        "<html><head><title>Brotli</title></head><body><p>compressed with brotli</p></body></html>";
    let mut compressed = Vec::new();
    {
        let mut writer = CompressorWriter::new(&mut compressed, 4096, 5, 22);
        let _ = writer.write_all(html.as_bytes());
    }
    HttpReply::new(200, "text/html; charset=utf-8", compressed)
        .with_header("Content-Encoding", "br")
}
