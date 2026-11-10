use crate::routes::decode_percent::decode_percent;
use crate::server::http_reply::HttpReply;
use crate::server::http_request::HttpRequest;

pub fn search_page(request: &HttpRequest) -> HttpReply {
    let query = request.path.split_once('?').map_or("", |(_, query)| query);
    let term = query
        .split('&')
        .find_map(|pair| pair.strip_prefix("q="))
        .map(decode_percent)
        .unwrap_or_default();
    let result = if term.is_empty() {
        "Type something to search.".to_owned()
    } else {
        format!("Results for {term}")
    };
    HttpReply::html(&format!(
        "<!DOCTYPE html><html><head><title>Search</title></head><body><h1>Search</h1>\
         <form action=\"/search\" method=\"get\"><input type=\"text\" name=\"q\"><input type=\"submit\" value=\"Go\"></form>\
         <p id=\"result\">{result}</p></body></html>"
    ))
}
