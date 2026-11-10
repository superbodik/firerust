use fr_net::body::charset_from_header::charset_from_header;
use fr_net::body::charset_from_markup::charset_from_markup;
use fr_net::body::decode_text::decode_text;

fn windows_1251(text: &str) -> Vec<u8> {
    encoding_rs::WINDOWS_1251.encode(text).0.into_owned()
}

#[test]
fn plain_utf8_is_decoded_as_is() {
    assert_eq!(decode_text("Привет".as_bytes(), None), "Привет");
}

#[test]
fn the_header_charset_decides() {
    let body = windows_1251("Привет, мир");

    assert_eq!(
        decode_text(&body, Some("text/html; charset=windows-1251")),
        "Привет, мир"
    );
}

#[test]
fn the_header_label_is_case_insensitive_and_may_be_quoted() {
    let body = windows_1251("мир");

    assert_eq!(
        decode_text(&body, Some("text/html; Charset=\"WINDOWS-1251\"")),
        "мир"
    );
}

#[test]
fn a_meta_charset_is_used_without_a_header() {
    let mut body = b"<html><head><meta charset=\"windows-1251\"></head><body>".to_vec();
    body.extend(windows_1251("Привет"));

    assert!(decode_text(&body, Some("text/html")).contains("Привет"));
}

#[test]
fn an_http_equiv_meta_charset_is_used() {
    let mut body =
        b"<meta http-equiv=\"Content-Type\" content=\"text/html; charset=koi8-r\"><p>".to_vec();
    body.extend(encoding_rs::KOI8_R.encode("мир").0.iter());

    assert!(decode_text(&body, None).contains("мир"));
}

#[test]
fn the_header_wins_over_the_meta_tag() {
    let mut body = b"<meta charset=\"windows-1251\">".to_vec();
    body.extend("Привет".as_bytes());

    assert!(decode_text(&body, Some("text/html; charset=utf-8")).contains("Привет"));
}

#[test]
fn a_byte_order_mark_wins_over_everything() {
    let mut body = vec![0xEF, 0xBB, 0xBF];
    body.extend("Привет".as_bytes());

    assert_eq!(
        decode_text(&body, Some("text/html; charset=windows-1251")),
        "Привет"
    );
}

#[test]
fn utf16_with_a_byte_order_mark_is_decoded() {
    let mut body = vec![0xFF, 0xFE];
    for unit in "hi".encode_utf16() {
        body.extend(unit.to_le_bytes());
    }

    assert_eq!(decode_text(&body, None), "hi");
}

#[test]
fn undeclared_invalid_utf8_falls_back_to_windows_1252() {
    let body = [b'c', b'a', b'f', 0xE9];

    assert_eq!(decode_text(&body, None), "café");
}

#[test]
fn a_css_charset_rule_is_honoured() {
    let mut body = b"@charset \"windows-1251\"; a:after { content: \"".to_vec();
    body.extend(windows_1251("мир"));
    body.extend(b"\" }");

    assert!(decode_text(&body, Some("text/css")).contains("мир"));
}

#[test]
fn an_unknown_label_is_ignored() {
    assert_eq!(
        decode_text("hello".as_bytes(), Some("text/html; charset=nonsense")),
        "hello"
    );
}

#[test]
fn header_charset_is_parsed_from_the_content_type() {
    assert_eq!(
        charset_from_header(Some("text/html; charset=utf-8")),
        Some(encoding_rs::UTF_8)
    );
    assert_eq!(charset_from_header(Some("text/html")), None);
    assert_eq!(charset_from_header(None), None);
}

#[test]
fn markup_charset_is_found_only_near_the_start() {
    let mut body = vec![b' '; 3000];
    body.extend(b"<meta charset=\"windows-1251\">");

    assert_eq!(charset_from_markup(&body), None);
}

#[test]
fn a_utf16_meta_declaration_is_read_as_utf8() {
    assert_eq!(
        charset_from_markup(b"<meta charset=\"utf-16\">"),
        Some(encoding_rs::UTF_8)
    );
}
