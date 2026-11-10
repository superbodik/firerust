use fr_dom::tree::shared_document::SharedDocument;
use fr_html::parse::parse_html::parse_html;
use url::Url;

use crate::load::page::Page;
use crate::prefetch::prefetched_page::PrefetchedPage;
use crate::prefetch::resource_store::ResourceStore;
use crate::script::start_scripts::start_scripts;
use crate::style::restyle::restyle;
use crate::text::document_title::document_title;

pub fn build_page(html: &str, url: Url) -> Page {
    build_page_with(html, url, &ResourceStore::new())
}

pub fn build_prefetched(prefetched: PrefetchedPage) -> Page {
    build_page_with(&prefetched.html, prefetched.url, &prefetched.store)
}

pub fn build_page_with(html: &str, url: Url, store: &ResourceStore) -> Page {
    let document = SharedDocument::new(parse_html(html));
    let runtime = start_scripts(&document, &url, store);
    let (styles, images) = restyle(&document.borrow(), &url, store);
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
