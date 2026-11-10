use std::path::PathBuf;

use fr_geom::color::rgba::Rgba;
use fr_net::address::address_error::AddressError;
use fr_net::load::net_error::NetError;
use fr_page::lines::line::Line;
use fr_page::load::load_page::load_page;
use fr_page::load::page_error::PageError;

fn pages_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/pages")
}

fn texts(lines: &[Line]) -> Vec<String> {
    lines.iter().map(Line::plain_text).collect()
}

#[test]
fn a_plain_page_is_read_to_lines() {
    let page = load_page("plain.html", &pages_dir()).expect("page");

    assert_eq!(page.title.as_deref(), Some("Plain page"));
    assert_eq!(
        texts(&page.lines),
        vec![
            "Hello firerust",
            "This is a plain page with bold and italic text.",
            "Second paragraph.",
        ]
    );
}

#[test]
fn nesting_scripts_and_lists_are_handled() {
    let page = load_page("nested.html", &pages_dir()).expect("page");

    assert_eq!(
        texts(&page.lines),
        vec!["Level three", "\u{2022} One", "\u{2022} Two"]
    );
}

#[test]
fn entities_and_line_breaks_are_handled() {
    let page = load_page("entities.html", &pages_dir()).expect("page");

    assert_eq!(
        texts(&page.lines),
        vec!["Fish & Chips <3 \u{a9} 2026", "Line one", "Line two"]
    );
}

#[test]
fn cyrillic_text_is_read() {
    let page = load_page("cyrillic.html", &pages_dir()).expect("page");

    assert_eq!(page.title.as_deref(), Some("Українська сторінка"));
    assert_eq!(
        texts(&page.lines),
        vec!["Привіт, світе!", "Це сторінка українською мовою."]
    );
}

#[test]
fn the_blog_reads_styles_from_three_places() {
    let page = load_page("blog.html", &pages_dir()).expect("page");

    assert_eq!(page.title.as_deref(), Some("Rust blog"));
    assert_eq!(
        texts(&page.lines),
        vec![
            "Rust blog",
            "Why a small browser",
            "Less memory is a feature.",
            "Read the plain page or the bold notes.",
            "Note: styles come from three places.",
            "\u{2022} External sheet",
            "\u{2022} Style element",
            "\u{2022} Inline attribute",
        ]
    );
}

#[test]
fn the_blog_external_sheet_sets_the_canvas_and_the_header() {
    let page = load_page("blog.html", &pages_dir()).expect("page");

    assert_eq!(page.background, Rgba::opaque(0xf4, 0xf1, 0xea));
    let header = &page.lines[0];
    assert_eq!(header.runs[0].style.color, Rgba::WHITE);
    assert_eq!(header.runs[0].style.font_size, 32.0);
    assert_eq!(
        header.background.map(|backdrop| backdrop.color),
        Some(Rgba::opaque(0xb7, 0x41, 0x0e))
    );
    assert_eq!(header.indent, 24.0);
}

#[test]
fn the_blog_style_element_and_inline_style_apply() {
    let page = load_page("blog.html", &pages_dir()).expect("page");

    let lead = &page.lines[2];
    assert_eq!(lead.runs[0].style.font_size, 20.0);
    assert_eq!(lead.runs[0].style.color, Rgba::opaque(0x44, 0x44, 0x44));

    let note = &page.lines[4];
    assert_eq!(note.runs[0].style.color, Rgba::opaque(0x8a, 0x6d, 0x3b));
    assert_eq!(
        note.background.map(|backdrop| backdrop.color),
        Some(Rgba::opaque(0xff, 0xf3, 0xcd))
    );
}

#[test]
fn the_blog_link_and_heading_take_author_colors() {
    let page = load_page("blog.html", &pages_dir()).expect("page");

    let heading = &page.lines[1];
    assert_eq!(heading.runs[0].style.color, Rgba::opaque(0xb7, 0x41, 0x0e));
    let paragraph = &page.lines[3];
    let link = paragraph
        .runs
        .iter()
        .find(|run| run.text.contains("plain page"))
        .expect("link run");
    assert_eq!(link.style.color, Rgba::opaque(0x0b, 0x53, 0x94));
}

#[test]
fn the_page_keeps_its_address() {
    let page = load_page("plain.html", &pages_dir()).expect("page");

    assert_eq!(page.url.scheme(), "file");
    assert!(page.url.path().ends_with("/plain.html"));
}

#[test]
fn a_missing_stylesheet_does_not_break_the_page() {
    let page = load_page("plain.html", &pages_dir()).expect("page");

    assert!(!page.lines.is_empty());
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
