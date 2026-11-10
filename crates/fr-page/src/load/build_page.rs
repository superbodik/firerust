use fr_dom::tree::shared_document::SharedDocument;
use fr_html::parse::parse_html::parse_html;
use url::Url;

use crate::load::page::Page;
use crate::script::start_scripts::start_scripts;
use crate::style::restyle::restyle;
use crate::text::document_title::document_title;

pub fn build_page(html: &str, url: Url) -> Page {
    let document = SharedDocument::new(parse_html(html));
    let runtime = start_scripts(&document, &url);
    let (styles, images) = restyle(&document.borrow(), &url);
    let title = document_title(&document.borrow());
    Page {
        title,
        url,
        document,
        styles,
        images,
        runtime,
    }
}
