use fr_net::body::decode_text::decode_text;
use fr_net::load::load_request::load_request;
use fr_net::load::net_error::NetError;
use fr_net::load::request::Request;
use fr_net::load::response::Response;

use crate::load::page_error::PageError;
use crate::prefetch::fetched::Fetched;
use crate::prefetch::prefetch_html::prefetch_html;
use crate::prefetch::prefetched_page::PrefetchedPage;
use crate::prefetch::resource_store::ResourceStore;
use crate::view::classify_response::classify_response;
use crate::view::image_view_html::image_view_html;
use crate::view::text_view_html::text_view_html;
use crate::view::view_kind::ViewKind;
use crate::view::view_title::view_title;

pub fn prefetch_request(request: &Request) -> Result<Fetched, PageError> {
    let response = load_request(request)?;
    if response.status >= 400 && response.body.is_empty() {
        return Err(PageError::Load(NetError::Status(response.status)));
    }
    Ok(match classify_response(&response) {
        ViewKind::Html => Fetched::Page(prefetch_html(response)),
        ViewKind::Text => Fetched::Page(text_page(response)),
        ViewKind::Image => Fetched::Page(image_page(response)),
        ViewKind::Download => Fetched::Download(response),
    })
}

fn text_page(response: Response) -> PrefetchedPage {
    let text = decode_text(&response.body, response.content_type.as_deref());
    PrefetchedPage {
        html: text_view_html(&view_title(&response.url), &text),
        url: response.url,
        store: ResourceStore::new(),
    }
}

fn image_page(response: Response) -> PrefetchedPage {
    let url = response.url.clone();
    let store = ResourceStore::new();
    let html = image_view_html(&url);
    store.insert(&url, response);
    PrefetchedPage { html, url, store }
}
