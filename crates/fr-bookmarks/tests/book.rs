use std::path::PathBuf;

use fr_bookmarks::model::bookmark::Bookmark;
use fr_bookmarks::storage::bookmark_book::{BookmarkBook, BookmarkChange};

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("firerust-book-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir.join("bookmarks.json")
}

#[test]
fn toggling_adds_then_removes() {
    let mut book = BookmarkBook::at(None);

    assert_eq!(book.toggle("A", "https://a.test/"), BookmarkChange::Added);
    assert!(book.contains("https://a.test/"));
    assert_eq!(book.toggle("A", "https://a.test/"), BookmarkChange::Removed);
    assert!(!book.contains("https://a.test/"));
}

#[test]
fn internal_and_local_pages_cannot_be_bookmarked() {
    let mut book = BookmarkBook::at(None);

    assert_eq!(book.toggle("x", "about:blank"), BookmarkChange::Unsupported);
    assert_eq!(
        book.toggle("x", "file:///C:/a.html"),
        BookmarkChange::Unsupported
    );
    assert!(book.list().is_empty());
}

#[test]
fn changes_are_saved_and_reopened() {
    let path = scratch("saved");
    {
        let mut book = BookmarkBook::at(Some(path.clone()));
        book.toggle("A", "https://a.test/");
        book.import(vec![Bookmark::new("B", "https://b.test/", "Folder")]);
    }

    let reopened = BookmarkBook::at(Some(path.clone()));

    assert_eq!(reopened.list().len(), 2);
    assert!(reopened.contains("https://b.test/"));
    if let Some(dir) = path.parent() {
        let _ = std::fs::remove_dir_all(dir);
    }
}

#[test]
fn removing_an_unknown_address_changes_nothing() {
    let mut book = BookmarkBook::at(None);
    book.toggle("A", "https://a.test/");

    assert!(!book.remove("https://other.test/"));
    assert!(book.remove("https://a.test/"));
}

#[test]
fn importing_reports_only_new_bookmarks() {
    let mut book = BookmarkBook::at(None);
    book.toggle("A", "https://a.test/");

    let added = book.import(vec![
        Bookmark::new("A", "https://a.test/", ""),
        Bookmark::new("B", "https://b.test/", ""),
    ]);

    assert_eq!(added, 1);
}
