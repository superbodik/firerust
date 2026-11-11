mod support;

use fr_download::name::file_name_for::file_name_for;
use fr_net::load::request::Request;
use fr_page::load::build_page::build_prefetched;
use fr_page::prefetch::fetched::Fetched;
use fr_page::prefetch::prefetch_request::prefetch_request;
use fr_test_server::server::test_server::TestServer;
use support::{layout_of, texts};
use url::Url;

fn fetch(server: &TestServer, path: &str) -> Fetched {
    let url = Url::parse(&server.url(path)).unwrap();
    prefetch_request(&Request::get(url)).expect("fetch")
}

#[test]
fn a_zip_over_http_is_a_download_with_every_byte() {
    let server = TestServer::start().unwrap();

    let Fetched::Download(response) = fetch(&server, "/archive.zip") else {
        panic!("expected a download");
    };

    assert_eq!(response.content_type.as_deref(), Some("application/zip"));
    assert_eq!(response.body.len(), 4008);
    assert_eq!(file_name_for(&response), "archive.zip");
}

#[test]
fn a_content_disposition_name_is_kept_for_the_saved_file() {
    let server = TestServer::start().unwrap();

    let Fetched::Download(response) = fetch(&server, "/report") else {
        panic!("expected a download");
    };

    assert_eq!(file_name_for(&response), "report 2026.txt");
    assert_eq!(response.body, b"saved from a header");
}

#[test]
fn a_text_file_over_http_is_shown_with_its_markup_literal() {
    let server = TestServer::start().unwrap();
    let page = fetch(&server, "/notes.txt").into_page().expect("a page");

    let page = build_prefetched(page);
    let shown = texts(&layout_of(&page, 600.0)).join("\n");

    assert!(shown.contains("    indented <b>line</b>"));
    assert!(shown.contains("third & last"));
}

#[test]
fn json_is_shown_as_text() {
    let server = TestServer::start().unwrap();
    let page = fetch(&server, "/data.json").into_page().expect("a page");

    let page = build_prefetched(page);

    assert!(
        texts(&layout_of(&page, 600.0))
            .join("")
            .contains("\"name\": \"firerust\"")
    );
}

#[test]
fn a_png_over_http_is_shown_as_a_picture() {
    let server = TestServer::start().unwrap();
    let page = fetch(&server, "/picture.png").into_page().expect("a page");

    let page = build_prefetched(page);

    assert_eq!(page.images.len(), 1);
}

#[test]
fn an_ordinary_page_is_still_a_page() {
    let server = TestServer::start().unwrap();

    assert!(fetch(&server, "/plain").into_page().is_some());
}

#[test]
fn the_links_page_carries_every_kind_of_link() {
    let server = TestServer::start().unwrap();
    let page = fetch(&server, "/links").into_page().expect("a page");

    assert!(page.html.contains("href=\"#details\""));
    assert!(page.html.contains("target=\"_blank\""));
    assert!(page.html.contains("mailto:"));
}
