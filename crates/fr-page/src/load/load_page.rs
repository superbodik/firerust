use std::path::Path;

use fr_html::parse::parse_html::parse_html;
use fr_net::address::resolve_address::resolve_address;
use fr_net::load::load_url::load_url;

use crate::load::page::Page;
use crate::load::page_error::PageError;
use crate::text::document_title::document_title;
use crate::text::text_lines::text_lines;

pub fn load_page(input: &str, base_dir: &Path) -> Result<Page, PageError> {
    let url = resolve_address(input, base_dir)?;
    let response = load_url(&url)?;
    let html = String::from_utf8_lossy(&response.body);
    let document = parse_html(&html);
    Ok(Page {
        url: response.url,
        title: document_title(&document),
        lines: text_lines(&document),
    })
}
