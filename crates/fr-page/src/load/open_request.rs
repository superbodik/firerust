use fr_net::body::decode_text::decode_text;
use fr_net::load::load_request::load_request;
use fr_net::load::net_error::NetError;
use fr_net::load::request::Request;

use crate::load::build_page::{build_page, build_page_with};
use crate::load::page::Page;
use crate::load::page_error::PageError;
use crate::prefetch::resource_store::ResourceStore;
use crate::view::classify_response::classify_response;
use crate::view::image_view_html::image_view_html;
use crate::view::text_view_html::text_view_html;
use crate::view::view_kind::ViewKind;
use crate::view::view_title::view_title;

pub fn open_request(request: &Request) -> Result<Page, PageError> {
    let response = load_request(request)?;
    if response.status >= 400 && response.body.is_empty() {
        return Err(PageError::Load(NetError::Status(response.status)));
    }
    match classify_response(&response) {
        ViewKind::Html => {
            let html = decode_text(&response.body, response.content_type.as_deref());
            Ok(build_page(&html, response.url))
        }
        ViewKind::Text => {
            let text = decode_text(&response.body, response.content_type.as_deref());
            let title = view_title(&response.url);
            Ok(build_page(&text_view_html(&title, &text), response.url))
        }
        ViewKind::Image => {
            let url = response.url.clone();
            let store = ResourceStore::new();
            let html = image_view_html(&url);
            store.insert(&url, response);
            Ok(build_page_with(&html, url, &store))
        }
        ViewKind::Download => Err(PageError::IsDownload),
    }
}
