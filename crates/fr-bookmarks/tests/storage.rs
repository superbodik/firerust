use std::path::PathBuf;

use fr_bookmarks::model::bookmark::Bookmark;
use fr_bookmarks::model::bookmark_list::BookmarkList;
use fr_bookmarks::storage::decode_bookmarks::decode_bookmarks;
use fr_bookmarks::storage::encode_bookmarks::encode_bookmarks;
use fr_bookmarks::storage::load_bookmarks::load_bookmarks;
use fr_bookmarks::storage::save_bookmarks::save_bookmarks;

fn scratch(name: &str) -> PathBuf {
    let dir =
        std::env::temp_dir().join(format!("firerust-bookmarks-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn sample() -> BookmarkList {
    let mut list = BookmarkList::new();
    list.add(Bookmark::new(
        "Rust \"lang\"",
        "https://rust-lang.org/",
        "Dev / Tools",
    ));
    list.add(Bookmark::new("Привет", "https://example.test/?a=1&b=2", ""));
    list
}

#[test]
fn encoding_and_decoding_round_trip() {
    let list = sample();

    let decoded = decode_bookmarks(&encode_bookmarks(&list)).expect("decoded");

    assert_eq!(decoded, list);
}

#[test]
fn text_that_is_not_a_bookmark_file_is_rejected() {
    assert!(decode_bookmarks("not json").is_none());
    assert!(decode_bookmarks("{\"other\": 1}").is_none());
}

#[test]
fn saving_then_loading_gives_the_same_list() {
    let dir = scratch("round");
    let path = dir.join("nested").join("bookmarks.json");

    save_bookmarks(&path, &sample()).expect("saved");

    assert_eq!(load_bookmarks(&path), sample());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_missing_file_gives_an_empty_list() {
    let dir = scratch("missing");

    assert!(load_bookmarks(&dir.join("none.json")).is_empty());
}

#[test]
fn a_broken_file_gives_an_empty_list() {
    let dir = scratch("broken");
    std::fs::create_dir_all(&dir).expect("dir");
    let path = dir.join("bookmarks.json");
    std::fs::write(&path, "{{{").expect("write");

    assert!(load_bookmarks(&path).is_empty());
    let _ = std::fs::remove_dir_all(&dir);
}
