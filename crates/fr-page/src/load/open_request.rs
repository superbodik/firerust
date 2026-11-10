use fr_net::body::decode_text::decode_text;
use fr_net::load::load_request::load_request;
use fr_net::load::net_error::NetError;
use fr_net::load::request::Request;

use crate::load::build_page::build_page;
use crate::load::page::Page;
use crate::load::page_error::PageError;

pub fn open_request(request: &Request) -> Result<Page, PageError> {
    let response = load_request(request)?;
    if response.status >= 400 && response.body.is_empty() {
        return Err(PageError::Load(NetError::Status(response.status)));
    }
    let html = decode_text(&response.body, response.content_type.as_deref());
    Ok(build_page(&html, response.url))
}
