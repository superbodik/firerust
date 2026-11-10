use crate::server::http_reply::HttpReply;

pub fn set_cookie_page() -> HttpReply {
    HttpReply::html("<html><body><p>cookie set</p></body></html>")
        .with_header("Set-Cookie", "theme=dark; Path=/")
        .with_header("Set-Cookie", "visit=1; Path=/show-cookie")
}
