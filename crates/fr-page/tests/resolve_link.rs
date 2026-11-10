use fr_page::link::resolve_link::resolve_link;
use url::Url;

fn base() -> Url {
    Url::parse("https://example.com/docs/page.html").unwrap()
}

#[test]
fn a_relative_link_is_joined_to_the_page_address() {
    let target = resolve_link(&base(), "other.html").unwrap();

    assert_eq!(target.as_str(), "https://example.com/docs/other.html");
}

#[test]
fn a_root_relative_link_keeps_only_the_host() {
    let target = resolve_link(&base(), "/top").unwrap();

    assert_eq!(target.as_str(), "https://example.com/top");
}

#[test]
fn an_absolute_link_is_kept() {
    let target = resolve_link(&base(), "http://rust-lang.org/").unwrap();

    assert_eq!(target.as_str(), "http://rust-lang.org/");
}

#[test]
fn a_parent_directory_link_is_normalised() {
    let target = resolve_link(&base(), "../index.html").unwrap();

    assert_eq!(target.as_str(), "https://example.com/index.html");
}

#[test]
fn a_fragment_only_link_goes_nowhere() {
    assert!(resolve_link(&base(), "#section").is_none());
}

#[test]
fn an_empty_link_goes_nowhere() {
    assert!(resolve_link(&base(), "  ").is_none());
}

#[test]
fn unsupported_schemes_are_refused() {
    assert!(resolve_link(&base(), "mailto:someone@example.com").is_none());
    assert!(resolve_link(&base(), "javascript:void(0)").is_none());
    assert!(resolve_link(&base(), "ftp://example.com/file").is_none());
}

#[test]
fn a_relative_link_on_a_local_page_stays_local() {
    let local = Url::parse("file:///C:/site/index.html").unwrap();

    let target = resolve_link(&local, "about.html").unwrap();

    assert_eq!(target.as_str(), "file:///C:/site/about.html");
}
