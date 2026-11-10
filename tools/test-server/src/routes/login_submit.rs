use crate::routes::parse_form::parse_form;
use crate::server::http_reply::HttpReply;
use crate::server::http_request::HttpRequest;

pub fn login_submit(request: &HttpRequest) -> HttpReply {
    let fields = parse_form(&String::from_utf8_lossy(&request.body));
    let value = |name: &str| {
        fields
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.clone())
            .unwrap_or_default()
    };
    if value("user") == "ada" && value("password") == "lovelace" {
        return HttpReply::redirect("/account")
            .with_header("Set-Cookie", "session=ada-ok; Path=/; HttpOnly");
    }
    HttpReply::new(
        401,
        "text/html; charset=utf-8",
        b"<html><body><h1>Wrong user or password</h1></body></html>".to_vec(),
    )
}
