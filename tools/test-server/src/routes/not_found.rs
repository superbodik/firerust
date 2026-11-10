use crate::server::http_reply::HttpReply;

pub fn not_found() -> HttpReply {
    HttpReply::new(
        404,
        "text/html; charset=utf-8",
        b"<html><body><h1>Not found</h1></body></html>".to_vec(),
    )
}
