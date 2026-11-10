use crate::server::http_reply::HttpReply;
use crate::server::http_request::HttpRequest;

pub fn show_cookie_page(request: &HttpRequest) -> HttpReply {
    let cookie = request.header("cookie").unwrap_or("none");
    HttpReply::html(&format!(
        "<html><body><p id=\"cookie\">cookie: {cookie}</p></body></html>"
    ))
}
