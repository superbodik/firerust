use crate::model::bookmark::Bookmark;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BookmarkList {
    items: Vec<Bookmark>,
}

impl BookmarkList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn items(&self) -> &[Bookmark] {
        &self.items
    }

    pub fn contains(&self, url: &str) -> bool {
        self.items.iter().any(|item| item.url == url)
    }

    pub fn add(&mut self, bookmark: Bookmark) -> bool {
        if bookmark.url.is_empty() || self.contains(&bookmark.url) {
            return false;
        }
        self.items.push(bookmark);
        true
    }

    pub fn remove(&mut self, url: &str) -> bool {
        let before = self.items.len();
        self.items.retain(|item| item.url != url);
        self.items.len() != before
    }

    pub fn merge(&mut self, incoming: Vec<Bookmark>) -> usize {
        incoming
            .into_iter()
            .map(|bookmark| self.add(bookmark))
            .filter(|added| *added)
            .count()
    }

    pub fn folders(&self) -> Vec<String> {
        let mut folders: Vec<String> = Vec::new();
        for item in &self.items {
            if !folders.contains(&item.folder) {
                folders.push(item.folder.clone());
            }
        }
        folders
    }
}
