use std::path::Path;

use crate::model::bookmark_list::BookmarkList;
use crate::storage::decode_bookmarks::decode_bookmarks;

pub fn load_bookmarks(path: &Path) -> BookmarkList {
    let Ok(text) = std::fs::read_to_string(path) else {
        return BookmarkList::new();
    };
    decode_bookmarks(&text).unwrap_or_else(|| {
        eprintln!("cannot read bookmarks from {}", path.display());
        BookmarkList::new()
    })
}
