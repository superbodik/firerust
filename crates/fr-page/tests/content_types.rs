mod support;

use std::path::PathBuf;

use fr_net::address::resolve_address::resolve_address;
use fr_net::load::request::Request;
use fr_net::load::response::Response;
use fr_page::load::build_page::build_prefetched;
use fr_page::load::load_page::load_page;
use fr_page::load::page_error::PageError;
use fr_page::prefetch::fetched::Fetched;
use fr_page::prefetch::prefetch_request::prefetch_request;
use fr_page::view::classify_response::classify_response;
use fr_page::view::view_kind::ViewKind;
use support::{layout_of, texts};
use url::Url;

fn pages_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/pages")
}

fn response(content_type: Option<&str>, disposition: Option<&str>, body: &[u8]) -> Response {
    Response {
        url: Url::parse("https://example.com/thing").unwrap(),
        status: 200,
        content_type: content_type.map(str::to_owned),
        content_disposition: disposition.map(str::to_owned),
        body: body.to_vec(),
    }
}

fn kind(content_type: Option<&str>) -> ViewKind {
    classify_response(&response(content_type, None, b"text"))
}

fn fetch(name: &str) -> Fetched {
    let url = resolve_address(name, &pages_dir()).expect("address");
    prefetch_request(&Request::get(url)).expect("fetch")
}

#[test]
fn html_and_unlabelled_text_are_pages() {
    assert_eq!(kind(Some("text/html; charset=utf-8")), ViewKind::Html);
    assert_eq!(kind(Some("application/xhtml+xml")), ViewKind::Html);
    assert_eq!(kind(None), ViewKind::Html);
}

#[test]
fn an_unlabelled_binary_body_is_a_download() {
    assert_eq!(
        classify_response(&response(None, None, b"PK\x03\x04\x00\x00")),
        ViewKind::Download
    );
}

#[test]
fn plain_text_json_and_xml_are_shown_as_text() {
    assert_eq!(kind(Some("text/plain")), ViewKind::Text);
    assert_eq!(kind(Some("text/css")), ViewKind::Text);
    assert_eq!(kind(Some("application/json")), ViewKind::Text);
    assert_eq!(kind(Some("application/vnd.api+json")), ViewKind::Text);
    assert_eq!(kind(Some("application/atom+xml")), ViewKind::Text);
    assert_eq!(kind(Some("image/svg+xml")), ViewKind::Text);
}

#[test]
fn decodable_images_are_shown_as_images() {
    for content_type in ["image/png", "image/jpeg", "image/gif", "image/webp"] {
        assert_eq!(kind(Some(content_type)), ViewKind::Image);
    }
}

#[test]
fn other_types_are_downloads() {
    for content_type in [
        "application/pdf",
        "application/zip",
        "application/octet-stream",
        "video/mp4",
        "image/avif",
        "application/vnd.ms-excel",
    ] {
        assert_eq!(
            kind(Some(content_type)),
            ViewKind::Download,
            "{content_type}"
        );
    }
}

#[test]
fn an_attachment_is_a_download_even_when_it_is_html() {
    let found = response(
        Some("text/html"),
        Some("attachment; filename=\"page.html\""),
        b"<p>x</p>",
    );

    assert_eq!(classify_response(&found), ViewKind::Download);
}

#[test]
fn an_inline_disposition_does_not_force_a_download() {
    let found = response(Some("text/plain"), Some("inline"), b"x");

    assert_eq!(classify_response(&found), ViewKind::Text);
}

#[test]
fn a_text_file_keeps_its_lines_and_shows_markup_literally() {
    let page = fetch("notes.txt").into_page().expect("a page");
    let page = build_prefetched(page);
    let shown = texts(&layout_of(&page, 600.0)).join("\n");

    assert!(shown.contains("first line"));
    assert!(shown.contains("  indented line"));
    assert!(shown.contains("<b>not bold</b> & more"));
    assert_eq!(page.title.as_deref(), Some("notes.txt"));
}

#[test]
fn an_image_file_becomes_a_page_with_that_image() {
    let page = fetch("gradient.png").into_page().expect("a page");
    let page = build_prefetched(page);

    assert_eq!(page.images.len(), 1);
    assert_eq!(page.title.as_deref(), Some("gradient.png"));
}

#[test]
fn a_binary_file_is_returned_as_a_download() {
    let Fetched::Download(response) = fetch("archive.zip") else {
        panic!("expected a download");
    };

    assert_eq!(response.content_type.as_deref(), Some("application/zip"));
    assert!(response.body.starts_with(b"PK"));
}

#[test]
fn opening_a_download_directly_is_reported() {
    let result = load_page("archive.zip", &pages_dir());

    assert!(matches!(result, Err(PageError::IsDownload)));
}

#[test]
fn opening_a_text_file_directly_gives_a_page() {
    let page = load_page("notes.txt", &pages_dir()).expect("page");

    assert!(
        texts(&layout_of(&page, 600.0))
            .join("\n")
            .contains("first line")
    );
}
