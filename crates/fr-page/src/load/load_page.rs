use std::path::Path;

use fr_net::address::resolve_address::resolve_address;
use fr_net::load::load_url::load_url;

use crate::load::build_page::build_page;
use crate::load::page::Page;
use crate::load::page_error::PageError;

pub fn load_page(input: &str, base_dir: &Path) -> Result<Page, PageError> {
    let url = resolve_address(input, base_dir)?;
    let response = load_url(&url)?;
    let html = String::from_utf8_lossy(&response.body);
    Ok(build_page(&html, response.url))
}
