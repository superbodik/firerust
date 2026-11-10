mod support;

use std::path::PathBuf;
use std::sync::Arc;

use fr_geom::color::rgba::Rgba;
use fr_net::address::address_error::AddressError;
use fr_net::load::net_error::NetError;
use fr_page::load::load_page::load_page;
use fr_page::load::page_error::PageError;
use support::{box_of_first, layout_of, texts};

fn pages_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/pages")
}

#[test]
fn a_plain_page_gives_its_paragraphs() {
    let page = load_page("plain.html", &pages_dir()).expect("page");

    assert_eq!(page.title.as_deref(), Some("Plain page"));
    assert_eq!(
        texts(&layout_of(&page, 800.0)),
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
        texts(&layout_of(&page, 800.0)),
        vec!["Level three", "One", "Two"]
    );
}

#[test]
fn entities_and_line_breaks_are_handled() {
    let page = load_page("entities.html", &pages_dir()).expect("page");

    assert_eq!(
        texts(&layout_of(&page, 800.0)),
        vec!["Fish & Chips <3 \u{a9} 2026", "Line one\nLine two"]
    );
}

#[test]
fn cyrillic_text_is_read() {
    let page = load_page("cyrillic.html", &pages_dir()).expect("page");

    assert_eq!(page.title.as_deref(), Some("Українська сторінка"));
    assert_eq!(
        texts(&layout_of(&page, 800.0)),
        vec!["Привіт, світе!", "Це сторінка українською мовою."]
    );
}

#[test]
fn the_blog_reads_styles_from_three_places() {
    let page = load_page("blog.html", &pages_dir()).expect("page");

    let tree = layout_of(&page, 800.0);
    assert_eq!(page.title.as_deref(), Some("Rust blog"));
    assert_eq!(
        texts(&tree),
        vec![
            "Rust blog",
            "Why a small browser",
            "Less memory is a feature.",
            "Read the plain page or the bold notes.",
            "Note: styles come from three places.",
            "External sheet",
            "Style element",
            "Inline attribute",
        ]
    );
    assert_eq!(tree.background, Rgba::opaque(0xf4, 0xf1, 0xea));
}

#[test]
fn the_blog_header_is_a_real_padded_box_with_a_background() {
    let page = load_page("blog.html", &pages_dir()).expect("page");

    let tree = layout_of(&page, 800.0);
    let header = box_of_first(&tree, &page.document, "header").expect("header box");
    assert_eq!(header.background, Rgba::opaque(0xb7, 0x41, 0x0e));
    assert_eq!(header.border_box.origin.x, 8.0);
    assert_eq!(header.border_box.size.width, 784.0);
    let heading = box_of_first(&tree, &page.document, "h1").expect("h1 box");
    assert_eq!(heading.border_box.origin.x, 24.0);
}

#[test]
fn images_are_downloaded_decoded_and_shared() {
    let page = load_page("images.html", &pages_dir()).expect("page");

    assert_eq!(page.images.len(), 4);
}

#[test]
fn the_same_image_address_is_decoded_once() {
    use fr_dom::tree::edge::Edge;

    let page = load_page("images.html", &pages_dir()).expect("page");

    let gradients: Vec<_> = page
        .document
        .walk(page.document.root())
        .filter_map(|edge| match edge {
            Edge::Enter(id) => {
                let element = page.document.node(id)?.kind().as_element()?;
                (element.attribute("src") == Some("gradient.png")).then_some(id)
            }
            Edge::Leave(_) => None,
        })
        .collect();
    assert_eq!(gradients.len(), 2);
    let first = page.images.get(gradients[0]).expect("first");
    let second = page.images.get(gradients[1]).expect("second");
    assert!(Arc::ptr_eq(first, second));
}

#[test]
fn a_page_with_images_lays_out_with_their_sizes() {
    let page = load_page("images.html", &pages_dir()).expect("page");

    let tree = layout_of(&page, 800.0);
    let first_image = box_of_first(&tree, &page.document, "img").expect("img box");
    assert_eq!(first_image.border_box.size.width, 240.0);
    assert_eq!(first_image.border_box.size.height, 120.0);
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
