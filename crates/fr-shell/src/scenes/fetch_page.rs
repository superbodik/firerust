use fr_net::address::resolve_address::resolve_address;
use fr_net::load::request::Request;
use fr_page::load::blank_page::{BLANK_ADDRESS, blank_page};
use fr_page::load::error_page::error_page;
use fr_page::load::open_request::open_request;
use fr_page::load::page::Page;

pub fn fetch_page(input: &str) -> Option<Page> {
    if input.trim() == BLANK_ADDRESS {
        return blank_page();
    }
    let base_dir = std::env::current_dir().unwrap_or_default();
    let url = resolve_address(input, &base_dir).ok()?;
    fetch_request(&Request::get(url))
}

pub fn fetch_request(request: &Request) -> Option<Page> {
    match open_request(request) {
        Ok(page) => Some(page),
        Err(error) => {
            eprintln!("cannot open {}: {error}", request.url);
            Some(error_page(request.url.clone(), &error.to_string()))
        }
    }
}
