use crate::server::http_reply::HttpReply;

pub fn zip_file() -> HttpReply {
    let mut bytes = b"PK\x03\x04\x00\x00\x00\x00".to_vec();
    bytes.extend((0..4000u32).map(|index| (index % 251) as u8));
    HttpReply::new(200, "application/zip", bytes)
}
