use std::path::Path;

use fr_net::address::resolve_address::resolve_address;
use fr_net::load::load_url::load_url;
use fr_net::load::net_error::NetError;
use url::Url;

fn base_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn a_local_file_is_read_whole() {
    let url = resolve_address("Cargo.toml", base_dir()).expect("url");

    let response = load_url(&url).expect("response");

    assert_eq!(response.status, 200);
    assert!(String::from_utf8_lossy(&response.body).contains("fr-net"));
}

#[test]
fn an_html_file_reports_its_content_type() {
    let url = resolve_address("../../tests/pages/plain.html", base_dir()).expect("url");

    let response = load_url(&url).expect("response");

    assert_eq!(response.content_type.as_deref(), Some("text/html"));
}

#[test]
fn a_missing_file_is_an_error() {
    let url = Url::from_file_path(base_dir().join("no-such-file.html")).expect("url");

    let result = load_url(&url);

    assert!(matches!(result, Err(NetError::FileRead(_))));
}

#[test]
fn an_unsupported_scheme_is_an_error() {
    let url = Url::parse("ftp://example.com/file").expect("url");

    let result = load_url(&url);

    assert!(matches!(result, Err(NetError::UnsupportedScheme(_))));
}
