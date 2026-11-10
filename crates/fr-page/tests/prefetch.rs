mod support;

use std::path::PathBuf;

use fr_net::address::resolve_address::resolve_address;
use fr_net::load::request::Request;
use fr_page::load::build_page::build_prefetched;
use fr_page::load::load_page::load_page;
use fr_page::prefetch::prefetch_request::prefetch_request;
use support::{layout_of, texts};

fn pages_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/pages")
}

fn prefetched(name: &str) -> fr_page::load::page::Page {
    let url = resolve_address(name, &pages_dir()).expect("address");
    let page = prefetch_request(&Request::get(url)).expect("prefetch");
    build_prefetched(page)
}

#[test]
fn a_prefetched_page_looks_like_a_page_loaded_directly() {
    let direct = load_page("blog.html", &pages_dir()).expect("direct");
    let ahead = prefetched("blog.html");

    assert_eq!(direct.title, ahead.title);
    assert_eq!(
        texts(&layout_of(&direct, 800.0)),
        texts(&layout_of(&ahead, 800.0))
    );
}

#[test]
fn a_prefetched_page_has_its_images() {
    let direct = load_page("images.html", &pages_dir()).expect("direct");
    let ahead = prefetched("images.html");

    assert!(!ahead.images.is_empty());
    assert_eq!(direct.images.len(), ahead.images.len());
}

#[test]
fn a_missing_page_is_an_error() {
    let url = resolve_address("missing-page.html", &pages_dir()).expect("address");

    assert!(prefetch_request(&Request::get(url)).is_err());
}
