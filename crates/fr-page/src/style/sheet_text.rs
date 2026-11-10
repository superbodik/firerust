use std::sync::Arc;

use fr_net::body::decode_text::decode_text;
use fr_net::load::load_url::load_url;
use url::Url;

use crate::prefetch::resource_store::ResourceStore;
use crate::style::shared_sheet_cache::{cached_sheet, remember_sheet};

pub fn sheet_text(url: &Url, store: &ResourceStore) -> Option<Arc<str>> {
    let response = match store.take(url) {
        Some(response) => response,
        None => {
            if let Some(text) = cached_sheet(url.as_str()) {
                return Some(text);
            }
            load_url(url).ok()?
        }
    };
    let text: Arc<str> = decode_text(&response.body, response.content_type.as_deref()).into();
    remember_sheet(url.as_str(), &text);
    Some(text)
}
