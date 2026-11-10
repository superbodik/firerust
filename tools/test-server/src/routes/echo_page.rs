use crate::server::http_reply::HttpReply;
use crate::server::http_request::HttpRequest;

pub fn echo_page(request: &HttpRequest) -> HttpReply {
    let query = request.path.split_once('?').map_or("", |(_, query)| query);
    let body = String::from_utf8_lossy(&request.body);
    let content_type = request.header("content-type").unwrap_or("none");
    HttpReply::html(&format!(
        "<html><body><p id=\"method\">{}</p><p id=\"query\">{query}</p><p id=\"body\">{body}</p><p id=\"type\">{content_type}</p></body></html>",
        request.method
    ))
}
