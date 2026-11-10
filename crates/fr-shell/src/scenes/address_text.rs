use fr_page::load::blank_page::BLANK_ADDRESS;
use url::Url;

pub fn address_text(url: &Url) -> String {
    if url.as_str() == BLANK_ADDRESS {
        String::new()
    } else {
        url.to_string()
    }
}
