use std::path::PathBuf;

use fr_bookmarks::import::browser_source::BrowserSource;
use fr_bookmarks::import::import_chrome::import_chrome;
use fr_bookmarks::import::import_error::ImportError;
use fr_bookmarks::import::import_from_file::import_from_file;
use fr_bookmarks::import::import_html::import_html;

const NETSCAPE: &str = "<!DOCTYPE NETSCAPE-Bookmark-file-1>\n\
<META HTTP-EQUIV=\"Content-Type\" CONTENT=\"text/html; charset=UTF-8\">\n\
<TITLE>Bookmarks</TITLE>\n\
<H1>Bookmarks</H1>\n\
<DL><p>\n\
    <DT><A HREF=\"https://top.test/\" ADD_DATE=\"1\">Top level</A>\n\
    <DT><H3>News</H3>\n\
    <DL><p>\n\
        <DT><A HREF=\"https://news.test/\">News site</A>\n\
        <DT><H3>World</H3>\n\
        <DL><p>\n\
            <DT><A HREF=\"https://world.test/\">World news</A>\n\
        </DL><p>\n\
        <DT><A HREF=\"https://news2.test/\">Second news</A>\n\
    </DL><p>\n\
    <DT><A HREF=\"javascript:alert(1)\">Script</A>\n\
    <DT><A HREF=\"place:sort=8\">Place</A>\n\
    <DT><A HREF=\"https://after.test/\">After folders</A>\n\
</DL><p>\n";

const CHROME: &str = r#"{
  "checksum": "x",
  "roots": {
    "bookmark_bar": {
      "type": "folder", "name": "Bookmarks bar",
      "children": [
        {"type": "url", "name": "Bar one", "url": "https://bar1.test/"},
        {"type": "folder", "name": "Dev", "children": [
          {"type": "url", "name": "Deep", "url": "https://deep.test/"},
          {"type": "url", "name": "Odd", "url": "chrome://settings"}
        ]},
        {"type": "url", "name": "Bar two", "url": "https://bar2.test/"}
      ]
    },
    "other": {
      "type": "folder", "name": "Other bookmarks",
      "children": [{"type": "url", "name": "Other one", "url": "https://other.test/"}]
    },
    "synced": {"type": "folder", "name": "Mobile bookmarks", "children": []}
  },
  "version": 1
}"#;

fn scratch_file(name: &str, content: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("firerust-import-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("dir");
    let path = dir.join(name);
    std::fs::write(&path, content).expect("write");
    path
}

fn summary(found: &[fr_bookmarks::model::bookmark::Bookmark]) -> Vec<(String, String)> {
    found
        .iter()
        .map(|item| (item.url.clone(), item.folder.clone()))
        .collect()
}

#[test]
fn a_netscape_file_gives_addresses_with_their_folders() {
    let found = import_html(NETSCAPE);

    assert_eq!(
        summary(&found),
        vec![
            ("https://top.test/".to_owned(), String::new()),
            ("https://news.test/".to_owned(), "News".to_owned()),
            ("https://world.test/".to_owned(), "News / World".to_owned()),
            ("https://news2.test/".to_owned(), "News".to_owned()),
            ("https://after.test/".to_owned(), String::new()),
        ]
    );
    assert_eq!(found[0].title, "Top level");
}

#[test]
fn unsafe_and_internal_addresses_are_skipped() {
    let found = import_html(NETSCAPE);

    assert!(found.iter().all(|item| item.url.starts_with("https://")));
}

#[test]
fn a_chrome_file_keeps_order_and_folders() {
    let found = import_chrome(CHROME).expect("imported");

    assert_eq!(
        summary(&found),
        vec![
            ("https://bar1.test/".to_owned(), "Bookmarks bar".to_owned()),
            (
                "https://deep.test/".to_owned(),
                "Bookmarks bar / Dev".to_owned()
            ),
            ("https://bar2.test/".to_owned(), "Bookmarks bar".to_owned()),
            (
                "https://other.test/".to_owned(),
                "Other bookmarks".to_owned()
            ),
        ]
    );
}

#[test]
fn json_without_roots_is_rejected() {
    assert!(import_chrome("{\"a\": 1}").is_none());
    assert!(import_chrome("nonsense").is_none());
}

#[test]
fn the_file_importer_picks_the_format_by_content() {
    let html = scratch_file("export.html", NETSCAPE);
    let json = scratch_file("Bookmarks", CHROME);

    assert_eq!(import_from_file(&html).expect("html").len(), 5);
    assert_eq!(import_from_file(&json).expect("json").len(), 4);
}

#[test]
fn a_missing_or_unrelated_file_is_an_error() {
    let missing = std::env::temp_dir().join("firerust-no-such-file.html");
    let text = scratch_file("notes.txt", "just some words");

    assert!(matches!(
        import_from_file(&missing),
        Err(ImportError::NotFound(_))
    ));
    assert!(matches!(
        import_from_file(&text),
        Err(ImportError::UnknownFormat)
    ));
}

#[test]
fn browser_sources_have_keys_labels_and_paths() {
    for source in BrowserSource::ALL {
        assert_eq!(BrowserSource::from_key(source.key()), Some(source));
        assert!(!source.label().is_empty());
    }
    assert_eq!(BrowserSource::from_key("netscape"), None);
}
