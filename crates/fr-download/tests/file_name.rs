use fr_download::name::file_name_for::file_name_for;
use fr_download::name::parse_content_disposition::parse_content_disposition;
use fr_download::name::sanitize_file_name::sanitize_file_name;
use fr_net::load::response::Response;
use url::Url;

fn response(address: &str, content_type: Option<&str>, disposition: Option<&str>) -> Response {
    Response {
        url: Url::parse(address).expect("address"),
        status: 200,
        content_type: content_type.map(str::to_owned),
        content_disposition: disposition.map(str::to_owned),
        body: Vec::new(),
    }
}

#[test]
fn an_attachment_header_gives_the_name() {
    let parsed = parse_content_disposition("attachment; filename=\"report 2026.pdf\"");

    assert!(parsed.is_attachment);
    assert_eq!(parsed.file_name.as_deref(), Some("report 2026.pdf"));
}

#[test]
fn an_inline_header_is_not_an_attachment() {
    let parsed = parse_content_disposition("inline");

    assert!(!parsed.is_attachment);
    assert_eq!(parsed.file_name, None);
}

#[test]
fn the_extended_name_wins_and_is_decoded() {
    let parsed = parse_content_disposition(
        "attachment; filename=\"fallback.txt\"; filename*=UTF-8''%D0%BE%D1%82%D1%87%D1%91%D1%82.txt",
    );

    assert_eq!(parsed.file_name.as_deref(), Some("отчёт.txt"));
}

#[test]
fn the_header_name_beats_the_address() {
    let found = response(
        "https://example.com/get?id=7",
        Some("application/zip"),
        Some("attachment; filename=archive.zip"),
    );

    assert_eq!(file_name_for(&found), "archive.zip");
}

#[test]
fn without_a_header_the_last_path_segment_is_used() {
    let found = response("https://example.com/files/My%20Book.pdf", None, None);

    assert_eq!(file_name_for(&found), "My Book.pdf");
}

#[test]
fn a_missing_extension_comes_from_the_content_type() {
    let found = response(
        "https://example.com/export",
        Some("text/csv; charset=utf-8"),
        None,
    );

    assert_eq!(file_name_for(&found), "export.csv");
}

#[test]
fn an_address_without_a_name_falls_back_to_download() {
    let found = response("https://example.com/", None, None);

    assert_eq!(file_name_for(&found), "download");
}

#[test]
fn path_tricks_and_forbidden_characters_are_removed() {
    assert_eq!(sanitize_file_name("..\\..\\windows\\evil.exe"), "evil.exe");
    assert_eq!(sanitize_file_name("a/b/c.txt"), "c.txt");
    assert_eq!(sanitize_file_name("na<me>:?.txt"), "name.txt");
    assert_eq!(sanitize_file_name("   "), "download");
    assert_eq!(sanitize_file_name(".."), "download");
    assert_eq!(sanitize_file_name("name. . "), "name");
}

#[test]
fn reserved_windows_names_get_a_prefix() {
    assert_eq!(sanitize_file_name("CON"), "_CON");
    assert_eq!(sanitize_file_name("nul.txt"), "_nul.txt");
    assert_eq!(sanitize_file_name("console.txt"), "console.txt");
}

#[test]
fn a_very_long_name_keeps_its_extension() {
    let long = format!("{}.pdf", "a".repeat(300));
    let cleaned = sanitize_file_name(&long);

    assert_eq!(cleaned.chars().count(), 120);
    assert!(cleaned.ends_with(".pdf"));
}
