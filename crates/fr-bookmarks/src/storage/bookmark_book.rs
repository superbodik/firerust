use std::path::PathBuf;

use crate::model::bookmark::Bookmark;
use crate::model::bookmark_list::BookmarkList;
use crate::model::is_bookmarkable::is_bookmarkable;
use crate::storage::bookmarks_path::bookmarks_path;
use crate::storage::load_bookmarks::load_bookmarks;
use crate::storage::save_bookmarks::save_bookmarks;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BookmarkChange {
    Added,
    Removed,
    Unsupported,
}

#[derive(Debug, Clone, Default)]
pub struct BookmarkBook {
    list: BookmarkList,
    path: Option<PathBuf>,
}

impl BookmarkBook {
    pub fn open() -> Self {
        Self::at(bookmarks_path())
    }

    pub fn at(path: Option<PathBuf>) -> Self {
        let list = match &path {
            Some(path) => load_bookmarks(path),
            None => BookmarkList::new(),
        };
        Self { list, path }
    }

    pub fn list(&self) -> &BookmarkList {
        &self.list
    }

    pub fn contains(&self, url: &str) -> bool {
        self.list.contains(url)
    }

    pub fn toggle(&mut self, title: &str, url: &str) -> BookmarkChange {
        if !is_bookmarkable(url) {
            return BookmarkChange::Unsupported;
        }
        let change = if self.list.contains(url) {
            self.list.remove(url);
            BookmarkChange::Removed
        } else {
            self.list.add(Bookmark::new(title, url, ""));
            BookmarkChange::Added
        };
        self.save();
        change
    }

    pub fn remove(&mut self, url: &str) -> bool {
        let removed = self.list.remove(url);
        if removed {
            self.save();
        }
        removed
    }

    pub fn import(&mut self, incoming: Vec<Bookmark>) -> usize {
        let added = self.list.merge(incoming);
        if added > 0 {
            self.save();
        }
        added
    }

    fn save(&self) {
        let Some(path) = &self.path else {
            return;
        };
        if let Err(error) = save_bookmarks(path, &self.list) {
            eprintln!("cannot save bookmarks to {}: {error}", path.display());
        }
    }
}
