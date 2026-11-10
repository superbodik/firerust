use std::path::PathBuf;

use fr_net::address::address_error::AddressError;
use fr_net::load::net_error::NetError;
use fr_page::load::load_page::load_page;
use fr_page::load::page_error::PageError;

fn pages_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/pages")
}

#[test]
fn a_plain_page_is_read_to_lines() {
    let page = load_page("plain.html", &pages_dir()).expect("page");

    assert_eq!(page.title.as_deref(), Some("Plain page"));
    assert_eq!(
        page.lines,
        vec![
            "Hello firerust",
            "",
            "This is a plain page with bold and italic text.",
            "",
            "Second paragraph.",
        ]
    );
}

#[test]
fn nesting_scripts_and_lists_are_handled() {
    let page = load_page("nested.html", &pages_dir()).expect("page");

    assert_eq!(
        page.lines,
        vec!["Level three", "", "\u{2022} One", "\u{2022} Two"]
    );
}

#[test]
fn entities_and_line_breaks_are_handled() {
    let page = load_page("entities.html", &pages_dir()).expect("page");

    assert_eq!(
        page.lines,
        vec!["Fish & Chips <3 \u{a9} 2026", "", "Line one", "Line two"]
    );
}

#[test]
fn cyrillic_text_is_read() {
    let page = load_page("cyrillic.html", &pages_dir()).expect("page");

    assert_eq!(page.title.as_deref(), Some("Українська сторінка"));
    assert_eq!(
        page.lines,
        vec!["Привіт, світе!", "", "Це сторінка українською мовою."]
    );
}

#[test]
fn the_page_keeps_its_address() {
    let page = load_page("plain.html", &pages_dir()).expect("page");

    assert_eq!(page.url.scheme(), "file");
    assert!(page.url.path().ends_with("/plain.html"));
}

#[test]
fn an_empty_address_is_an_error() {
    let result = load_page("  ", &pages_dir());

    assert!(matches!(
        result,
        Err(PageError::Address(AddressError::Empty))
    ));
}

#[test]
fn a_missing_file_url_is_an_error() {
    let missing = format!(
        "file:///{}",
        pages_dir()
            .join("no-such-page.html")
            .to_string_lossy()
            .replace('\\', "/")
    );

    let result = load_page(&missing, &pages_dir());

    assert!(matches!(
        result,
        Err(PageError::Load(NetError::FileRead(_)))
    ));
}
