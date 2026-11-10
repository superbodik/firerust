use std::io;
use std::path::Path;

use crate::model::bookmark_list::BookmarkList;
use crate::storage::encode_bookmarks::encode_bookmarks;

pub fn save_bookmarks(path: &Path, list: &BookmarkList) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let temporary = path.with_extension("json.tmp");
    std::fs::write(&temporary, encode_bookmarks(list))?;
    std::fs::rename(&temporary, path)
}
