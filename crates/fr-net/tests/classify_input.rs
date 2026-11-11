use std::path::Path;

use fr_net::address::classify_input::classify_input;
use fr_net::address::input_kind::InputKind;

fn kind(input: &str) -> InputKind {
    classify_input(input, Path::new(env!("CARGO_MANIFEST_DIR")))
}

#[test]
fn full_addresses_stay_addresses() {
    assert_eq!(kind("https://example.com"), InputKind::Address);
    assert_eq!(kind("http://127.0.0.1:8080/links"), InputKind::Address);
    assert_eq!(kind("file:///C:/Windows/win.ini"), InputKind::Address);
    assert_eq!(kind("about:menu"), InputKind::Address);
}

#[test]
fn a_host_without_a_scheme_is_an_address() {
    assert_eq!(kind("example.com"), InputKind::Address);
    assert_eq!(kind("news.ycombinator.com/newest"), InputKind::Address);
    assert_eq!(kind("example.com:8080/path?x=1#top"), InputKind::Address);
    assert_eq!(kind("docs.rs"), InputKind::Address);
    assert_eq!(kind("пример.рф"), InputKind::Address);
}

#[test]
fn localhost_and_numbers_are_addresses() {
    assert_eq!(kind("localhost"), InputKind::Address);
    assert_eq!(kind("localhost:3000"), InputKind::Address);
    assert_eq!(kind("192.168.0.1"), InputKind::Address);
    assert_eq!(kind("[::1]:8080"), InputKind::Address);
}

#[test]
fn a_single_word_is_a_search() {
    assert_eq!(kind("rust"), InputKind::Search);
    assert_eq!(kind("рубль"), InputKind::Search);
    assert_eq!(kind("юткуб"), InputKind::Search);
}

#[test]
fn words_with_spaces_are_a_search() {
    assert_eq!(kind("how to cook rice"), InputKind::Search);
    assert_eq!(kind("example.com login"), InputKind::Search);
    assert_eq!(kind("курс доллара сегодня"), InputKind::Search);
}

#[test]
fn a_dot_alone_does_not_make_a_host() {
    assert_eq!(kind("version 1.2"), InputKind::Search);
    assert_eq!(kind("a."), InputKind::Search);
    assert_eq!(kind(".com"), InputKind::Search);
    assert_eq!(kind("what?"), InputKind::Search);
}

#[test]
fn a_path_on_disk_is_an_address() {
    assert_eq!(kind("C:\\Users\\me\\page.html"), InputKind::Address);
    assert_eq!(kind("Cargo.toml"), InputKind::Address);
}

#[test]
fn an_empty_input_is_left_to_the_address_logic() {
    assert_eq!(kind("   "), InputKind::Address);
}
