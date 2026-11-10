#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bookmark {
    pub title: String,
    pub url: String,
    pub folder: String,
}

impl Bookmark {
    pub fn new(title: &str, url: &str, folder: &str) -> Self {
        let title = title.trim();
        Self {
            title: if title.is_empty() {
                url.to_owned()
            } else {
                title.to_owned()
            },
            url: url.trim().to_owned(),
            folder: folder.trim().to_owned(),
        }
    }
}
