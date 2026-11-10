use fr_html::parse::parse_html::parse_html;
use fr_net::body::decode_text::decode_text;
use fr_net::load::load_request::load_request;
use fr_net::load::net_error::NetError;
use fr_net::load::request::Request;
use fr_security::guard::page_guard::PageGuard;

use crate::load::page_error::PageError;
use crate::prefetch::fetch_in_parallel::fetch_in_parallel;
use crate::prefetch::prefetched_page::PrefetchedPage;
use crate::prefetch::resource_store::ResourceStore;
use crate::prefetch::subresource_urls::subresource_urls;

pub fn prefetch_request(request: &Request) -> Result<PrefetchedPage, PageError> {
    let response = load_request(request)?;
    if response.status >= 400 && response.body.is_empty() {
        return Err(PageError::Load(NetError::Status(response.status)));
    }
    let html = decode_text(&response.body, response.content_type.as_deref());
    let url = response.url;
    let store = ResourceStore::new();
    let document = parse_html(&html);
    let guard = PageGuard::new(&document, &url);
    fetch_in_parallel(subresource_urls(&document, &url, &guard), &store);
    Ok(PrefetchedPage { html, url, store })
}
