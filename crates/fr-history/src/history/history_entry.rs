use url::Url;

#[derive(Debug, Clone, PartialEq)]
pub struct HistoryEntry {
    pub url: Url,
    pub scroll: f32,
}

impl HistoryEntry {
    pub fn new(url: Url) -> Self {
        Self { url, scroll: 0.0 }
    }
}
