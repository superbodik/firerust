use crate::server::http_reply::HttpReply;

pub fn plain_page() -> HttpReply {
    HttpReply::html(
        "<!DOCTYPE html><html><head><title>Plain</title></head><body><h1>Hello from the test server</h1><p><a href=\"/login\">Log in</a> or <a href=\"/search\">search</a></p></body></html>",
    )
}
