use fr_theme::colors::palette_for::palette_for;
use fr_theme::kind::theme_kind::ThemeKind;
use fr_theme::page::internal_css::internal_css;
use url::Url;

use crate::load::build_page::build_page;
use crate::load::escape_html::escape_html;
use crate::load::page::Page;

pub fn error_page(url: Url, message: &str) -> Page {
    error_page_for(url, message, ThemeKind::Light)
}

pub fn error_page_for(url: Url, message: &str, theme: ThemeKind) -> Page {
    let css = internal_css(palette_for(theme));
    let html = format!(
        "<html><head><title>Cannot open page</title><style>{css}</style></head><body>\
         <div class=\"page\">\
         <h1>Cannot open this page</h1>\
         <p class=\"note\">{}</p>\
         <div class=\"card\"><p>{}</p></div>\
         <p class=\"note\">Check the address, your connection, and try again with F5.</p>\
         </div></body></html>",
        escape_html(url.as_str()),
        escape_html(message),
    );
    build_page(&html, url)
}
