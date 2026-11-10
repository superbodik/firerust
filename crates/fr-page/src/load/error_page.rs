use url::Url;

use crate::load::build_page::build_page;
use crate::load::escape_html::escape_html;
use crate::load::page::Page;

pub fn error_page(url: Url, message: &str) -> Page {
    let html = format!(
        "<html><head><title>Cannot open page</title></head><body style=\"background-color: #f4f1ea\">\
         <h1 style=\"color: #b7410e\">Cannot open this page</h1>\
         <p>{}</p><p>{}</p></body></html>",
        escape_html(url.as_str()),
        escape_html(message),
    );
    build_page(&html, url)
}
