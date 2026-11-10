use fr_net::address::resolve_address::resolve_address;
use fr_page::load::error_page::error_page;
use fr_page::load::load_page::load_page;
use fr_page::load::page::Page;

pub fn fetch_page(input: &str) -> Option<Page> {
    let base_dir = std::env::current_dir().unwrap_or_default();
    match load_page(input, &base_dir) {
        Ok(page) => Some(page),
        Err(error) => {
            eprintln!("cannot open {input}: {error}");
            let url = resolve_address(input, &base_dir).ok()?;
            Some(error_page(url, &error.to_string()))
        }
    }
}
