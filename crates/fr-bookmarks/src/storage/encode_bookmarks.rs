use serde_json::{Value, json};

use crate::model::bookmark_list::BookmarkList;

pub fn encode_bookmarks(list: &BookmarkList) -> String {
    let items: Vec<Value> = list
        .items()
        .iter()
        .map(|bookmark| {
            json!({
                "title": bookmark.title,
                "url": bookmark.url,
                "folder": bookmark.folder,
            })
        })
        .collect();
    let document = json!({ "version": 1, "bookmarks": items });
    serde_json::to_string_pretty(&document).unwrap_or_default()
}
