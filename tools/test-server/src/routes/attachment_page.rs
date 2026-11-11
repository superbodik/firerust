use crate::server::http_reply::HttpReply;

pub fn attachment_page() -> HttpReply {
    HttpReply::new(
        200,
        "text/plain; charset=utf-8",
        b"saved from a header".to_vec(),
    )
    .with_header(
        "Content-Disposition",
        "attachment; filename=\"report 2026.txt\"",
    )
}
