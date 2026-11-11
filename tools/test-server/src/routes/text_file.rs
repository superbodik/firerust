use crate::server::http_reply::HttpReply;

pub fn text_file() -> HttpReply {
    HttpReply::new(
        200,
        "text/plain; charset=utf-8",
        "first line\n    indented <b>line</b>\nthird & last\n"
            .as_bytes()
            .to_vec(),
    )
}
