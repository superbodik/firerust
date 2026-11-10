use url::Url;

use crate::load::build_page::build_page;
use crate::load::page::Page;

pub const BLANK_ADDRESS: &str = "about:blank";

pub fn blank_page() -> Option<Page> {
    let url = Url::parse(BLANK_ADDRESS).ok()?;
    let html = "<html><head><title>New tab</title></head>\
                <body style=\"background-color: #f4f1ea\">\
                <h1 style=\"color: #b7410e\">New tab</h1>\
                <p>Type an address in the bar above and press Enter.</p>\
                </body></html>";
    Some(build_page(html, url))
}
