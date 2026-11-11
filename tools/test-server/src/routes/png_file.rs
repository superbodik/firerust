use crate::server::http_reply::HttpReply;

const GRADIENT: &[u8] = include_bytes!("../../../../tests/pages/gradient.png");

pub fn png_file() -> HttpReply {
    HttpReply::new(200, "image/png", GRADIENT.to_vec())
}
