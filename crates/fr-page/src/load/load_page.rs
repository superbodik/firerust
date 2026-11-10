use std::path::Path;

use fr_net::address::resolve_address::resolve_address;
use fr_net::load::request::Request;

use crate::load::open_request::open_request;
use crate::load::page::Page;
use crate::load::page_error::PageError;

pub fn load_page(input: &str, base_dir: &Path) -> Result<Page, PageError> {
    let url = resolve_address(input, base_dir)?;
    open_request(&Request::get(url))
}
