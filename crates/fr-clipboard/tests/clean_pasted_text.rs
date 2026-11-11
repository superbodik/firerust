use fr_clipboard::text::clean_pasted_text::clean_pasted_text;

#[test]
fn line_breaks_and_tabs_become_spaces() {
    assert_eq!(
        clean_pasted_text("one\ntwo\r\nthree\tfour"),
        "one two  three four"
    );
}

#[test]
fn other_control_characters_are_dropped() {
    assert_eq!(clean_pasted_text("a\u{7}b\u{0}c"), "abc");
}

#[test]
fn ordinary_and_non_latin_text_is_kept() {
    assert_eq!(
        clean_pasted_text("https://example.com/путь?q=1"),
        "https://example.com/путь?q=1"
    );
}

#[test]
fn a_huge_paste_is_cut() {
    let huge = "x".repeat(50_000);

    assert_eq!(clean_pasted_text(&huge).chars().count(), 10_000);
}
