use serde_json::Value;

use crate::model::bookmark::Bookmark;
use crate::model::is_bookmarkable::is_bookmarkable;

pub fn import_chrome(text: &str) -> Option<Vec<Bookmark>> {
    let document: Value = serde_json::from_str(text).ok()?;
    let roots = document.get("roots")?.as_object()?;
    let mut found = Vec::new();
    let mut pending: Vec<(&Value, String)> = Vec::new();
    for key in ["synced", "other", "bookmark_bar"] {
        if let Some(root) = roots.get(key) {
            pending.push((root, String::new()));
        }
    }
    while let Some((node, folder)) = pending.pop() {
        let kind = node.get("type").and_then(Value::as_str).unwrap_or("");
        if kind == "url" {
            let url = node.get("url").and_then(Value::as_str).unwrap_or("");
            let title = node.get("name").and_then(Value::as_str).unwrap_or("");
            if is_bookmarkable(url) {
                found.push(Bookmark::new(title, url, &folder));
            }
            continue;
        }
        let Some(children) = node.get("children").and_then(Value::as_array) else {
            continue;
        };
        let name = node.get("name").and_then(Value::as_str).unwrap_or("");
        let inner = join_folder(&folder, name);
        for child in children.iter().rev() {
            pending.push((child, inner.clone()));
        }
    }
    Some(found)
}

fn join_folder(parent: &str, name: &str) -> String {
    match (parent.is_empty(), name.is_empty()) {
        (true, _) => name.to_owned(),
        (false, true) => parent.to_owned(),
        (false, false) => format!("{parent} / {name}"),
    }
}
