use fr_dom::tree::document::Document;
use fr_theme::colors::palette_for::palette_for;
use fr_theme::kind::theme_kind::ThemeKind;
use fr_theme::page::reader_css::reader_css;
use url::Url;

use crate::load::build_page::build_page;
use crate::load::escape_html::escape_html;
use crate::load::page::Page;
use crate::reader::extract_reader_content::extract_reader_content;

pub fn build_reader_page(document: &Document, url: &Url, theme: ThemeKind) -> Option<Page> {
    let content = extract_reader_content(document)?;
    let css = reader_css(palette_for(theme));
    let origin = url.host_str().unwrap_or("this page");
    let title = escape_html(&content.title);
    let heading = if content.html.trim_start().starts_with("<h1") {
        String::new()
    } else {
        format!("<h1>{title}</h1>")
    };
    let html = format!(
        "<html><head><title>{title}</title><style>{css}</style></head><body>\
         <div class=\"reader\"><p class=\"origin\">{}</p>{heading}{}</div></body></html>",
        escape_html(origin),
        content.html
    );
    Some(build_page(&html, url.clone()))
}
