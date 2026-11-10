use serde_json::Value;

use crate::model::bookmark::Bookmark;
use crate::model::bookmark_list::BookmarkList;

pub fn decode_bookmarks(text: &str) -> Option<BookmarkList> {
    let document: Value = serde_json::from_str(text).ok()?;
    let items = document.get("bookmarks")?.as_array()?;
    let mut list = BookmarkList::new();
    for item in items {
        let url = item.get("url").and_then(Value::as_str).unwrap_or("");
        let title = item.get("title").and_then(Value::as_str).unwrap_or("");
        let folder = item.get("folder").and_then(Value::as_str).unwrap_or("");
        list.add(Bookmark::new(title, url, folder));
    }
    Some(list)
}
