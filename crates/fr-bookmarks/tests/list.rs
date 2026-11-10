use fr_bookmarks::model::bookmark::Bookmark;
use fr_bookmarks::model::bookmark_list::BookmarkList;
use fr_bookmarks::model::is_bookmarkable::is_bookmarkable;

fn mark(title: &str, url: &str) -> Bookmark {
    Bookmark::new(title, url, "")
}

#[test]
fn a_new_bookmark_is_added_once() {
    let mut list = BookmarkList::new();

    assert!(list.add(mark("Rust", "https://rust-lang.org/")));
    assert!(!list.add(mark("Again", "https://rust-lang.org/")));
    assert_eq!(list.len(), 1);
    assert!(list.contains("https://rust-lang.org/"));
}

#[test]
fn a_bookmark_without_a_title_uses_its_address() {
    let bookmark = Bookmark::new("  ", "https://a.test/", "");

    assert_eq!(bookmark.title, "https://a.test/");
}

#[test]
fn a_bookmark_without_an_address_is_ignored() {
    let mut list = BookmarkList::new();

    assert!(!list.add(mark("Nothing", "")));
    assert!(list.is_empty());
}

#[test]
fn removing_works_by_address() {
    let mut list = BookmarkList::new();
    list.add(mark("A", "https://a.test/"));
    list.add(mark("B", "https://b.test/"));

    assert!(list.remove("https://a.test/"));
    assert!(!list.remove("https://a.test/"));
    assert_eq!(list.len(), 1);
}

#[test]
fn merging_counts_only_the_new_ones() {
    let mut list = BookmarkList::new();
    list.add(mark("A", "https://a.test/"));

    let added = list.merge(vec![
        mark("A again", "https://a.test/"),
        mark("B", "https://b.test/"),
        mark("C", "https://c.test/"),
    ]);

    assert_eq!(added, 2);
    assert_eq!(list.len(), 3);
}

#[test]
fn folders_keep_their_first_seen_order() {
    let mut list = BookmarkList::new();
    list.add(Bookmark::new("A", "https://a.test/", "News"));
    list.add(Bookmark::new("B", "https://b.test/", ""));
    list.add(Bookmark::new("C", "https://c.test/", "News"));

    assert_eq!(list.folders(), vec!["News".to_owned(), String::new()]);
}

#[test]
fn only_web_addresses_can_be_bookmarked() {
    assert!(is_bookmarkable("https://a.test/"));
    assert!(is_bookmarkable("HTTP://a.test/"));
    assert!(!is_bookmarkable("javascript:alert(1)"));
    assert!(!is_bookmarkable("about:blank"));
    assert!(!is_bookmarkable("file:///C:/a.html"));
}
