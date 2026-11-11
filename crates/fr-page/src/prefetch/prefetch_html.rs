use fr_html::parse::parse_html::parse_html;
use fr_net::body::decode_text::decode_text;
use fr_net::load::response::Response;
use fr_security::guard::page_guard::PageGuard;

use crate::prefetch::fetch_in_parallel::fetch_in_parallel;
use crate::prefetch::prefetched_page::PrefetchedPage;
use crate::prefetch::resource_store::ResourceStore;
use crate::prefetch::subresource_urls::subresource_urls;

pub fn prefetch_html(response: Response) -> PrefetchedPage {
    let html = decode_text(&response.body, response.content_type.as_deref());
    let url = response.url;
    let store = ResourceStore::new();
    let document = parse_html(&html);
    let guard = PageGuard::new(&document, &url);
    fetch_in_parallel(subresource_urls(&document, &url, &guard), &store);
    PrefetchedPage { html, url, store }
}
