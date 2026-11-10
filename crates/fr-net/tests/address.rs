use std::path::Path;

use fr_net::address::address_error::AddressError;
use fr_net::address::resolve_address::resolve_address;

fn base_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn a_full_address_is_kept() {
    let url = resolve_address("https://example.com/a?b=1", base_dir()).expect("url");

    assert_eq!(url.as_str(), "https://example.com/a?b=1");
}

#[test]
fn a_bare_host_gets_https() {
    let url = resolve_address("example.com", base_dir()).expect("url");

    assert_eq!(url.as_str(), "https://example.com/");
}

#[test]
fn a_host_with_a_port_gets_https() {
    let url = resolve_address("example.com:8080/x", base_dir()).expect("url");

    assert_eq!(url.as_str(), "https://example.com:8080/x");
}

#[test]
fn surrounding_spaces_are_ignored() {
    let url = resolve_address("  https://example.com/  ", base_dir()).expect("url");

    assert_eq!(url.as_str(), "https://example.com/");
}

#[test]
fn an_empty_address_is_refused() {
    assert_eq!(resolve_address("   ", base_dir()), Err(AddressError::Empty));
}

#[test]
fn an_unknown_scheme_is_refused() {
    assert_eq!(
        resolve_address("ftp://example.com/file", base_dir()),
        Err(AddressError::UnsupportedScheme("ftp".to_owned()))
    );
}

#[test]
fn an_existing_relative_file_becomes_a_file_address() {
    let url = resolve_address("Cargo.toml", base_dir()).expect("url");

    assert_eq!(url.scheme(), "file");
    assert!(url.path().ends_with("/Cargo.toml"));
}

#[test]
fn a_missing_file_is_treated_as_a_host() {
    let url = resolve_address("missing-page.example", base_dir()).expect("url");

    assert_eq!(url.as_str(), "https://missing-page.example/");
}
