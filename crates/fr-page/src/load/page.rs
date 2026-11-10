use url::Url;

#[derive(Debug, Clone)]
pub struct Page {
    pub url: Url,
    pub title: Option<String>,
    pub lines: Vec<String>,
}
