use fr_css::cascade::compute_styles::compute_styles;
use fr_css::defaults::user_agent_sheet::user_agent_sheet;
use fr_html::parse::parse_html::parse_html;
use url::Url;

use crate::images::collect_images::collect_images;
use crate::load::page::Page;
use crate::style::collect_author_sheets::collect_author_sheets;
use crate::text::document_title::document_title;

pub fn build_page(html: &str, url: Url) -> Page {
    let document = parse_html(html);
    let author_sheets = collect_author_sheets(&document, &url);
    let styles = compute_styles(&document, &user_agent_sheet(), &author_sheets);
    let images = collect_images(&document, &styles, &url);
    Page {
        title: document_title(&document),
        url,
        document,
        styles,
        images,
    }
}
