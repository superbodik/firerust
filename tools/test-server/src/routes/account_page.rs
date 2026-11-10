use crate::server::http_reply::HttpReply;
use crate::server::http_request::HttpRequest;

pub fn account_page(request: &HttpRequest) -> HttpReply {
    if request.cookie("session").as_deref() == Some("ada-ok") {
        return HttpReply::html(
            "<html><head><title>Account</title></head><body><h1>Welcome back, ada</h1></body></html>",
        );
    }
    HttpReply::redirect("/login")
}
