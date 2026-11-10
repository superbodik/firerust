use std::collections::HashMap;
use std::sync::Mutex;

use fr_net::load::load_url::load_url;
use fr_net::load::response::Response;
use url::Url;

#[derive(Debug, Default)]
pub struct ResourceStore {
    entries: Mutex<HashMap<String, Response>>,
}

impl ResourceStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&self, url: &Url, response: Response) {
        if let Ok(mut entries) = self.entries.lock() {
            entries.insert(url.as_str().to_owned(), response);
        }
    }

    pub fn take(&self, url: &Url) -> Option<Response> {
        self.entries.lock().ok()?.remove(url.as_str())
    }

    pub fn load(&self, url: &Url) -> Option<Response> {
        self.take(url).or_else(|| load_url(url).ok())
    }
}
