use crate::routes::account_page::account_page;
use crate::routes::brotli_page::brotli_page;
use crate::routes::cp1251_page::cp1251_page;
use crate::routes::echo_page::echo_page;
use crate::routes::gzip_page::gzip_page;
use crate::routes::login_page::login_page;
use crate::routes::login_submit::login_submit;
use crate::routes::meta_cp1251_page::meta_cp1251_page;
use crate::routes::not_found::not_found;
use crate::routes::plain_page::plain_page;
use crate::routes::search_page::search_page;
use crate::routes::set_cookie_page::set_cookie_page;
use crate::routes::show_cookie_page::show_cookie_page;
use crate::server::http_reply::HttpReply;
use crate::server::http_request::HttpRequest;

pub fn route(request: &HttpRequest) -> HttpReply {
    let path = request.path.split('?').next().unwrap_or("/");
    match (request.method.as_str(), path) {
        ("GET", "/" | "/plain") => plain_page(),
        ("GET", "/cp1251") => cp1251_page(),
        ("GET", "/meta-cp1251") => meta_cp1251_page(),
        ("GET", "/gzip") => gzip_page(),
        ("GET", "/brotli") => brotli_page(),
        ("GET", "/redirect") => HttpReply::redirect("/plain"),
        ("GET", "/redirect-twice") => HttpReply::redirect("/redirect"),
        ("GET", "/set-cookie") => set_cookie_page(),
        ("GET", "/show-cookie") => show_cookie_page(request),
        ("GET", "/login") => login_page(),
        ("POST", "/login") => login_submit(request),
        ("GET", "/account") => account_page(request),
        ("GET" | "POST", "/echo") => echo_page(request),
        ("GET", "/search") => search_page(request),
        _ => not_found(),
    }
}
