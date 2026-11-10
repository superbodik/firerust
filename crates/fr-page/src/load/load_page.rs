use std::path::Path;

use fr_css::cascade::compute_styles::compute_styles;
use fr_css::defaults::user_agent_sheet::user_agent_sheet;
use fr_html::parse::parse_html::parse_html;
use fr_net::address::resolve_address::resolve_address;
use fr_net::load::load_url::load_url;

use crate::lines::canvas_background::canvas_background;
use crate::lines::page_lines::page_lines;
use crate::load::page::Page;
use crate::load::page_error::PageError;
use crate::style::collect_author_sheets::collect_author_sheets;
use crate::text::document_title::document_title;

pub fn load_page(input: &str, base_dir: &Path) -> Result<Page, PageError> {
    let url = resolve_address(input, base_dir)?;
    let response = load_url(&url)?;
    let html = String::from_utf8_lossy(&response.body);
    let document = parse_html(&html);
    let author_sheets = collect_author_sheets(&document, &response.url);
    let styles = compute_styles(&document, &user_agent_sheet(), &author_sheets);
    Ok(Page {
        title: document_title(&document),
        background: canvas_background(&document, &styles),
        lines: page_lines(&document, &styles),
        url: response.url,
    })
}
