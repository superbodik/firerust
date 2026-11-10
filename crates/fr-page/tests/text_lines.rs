use fr_html::parse::parse_html::parse_html;
use fr_page::text::document_title::document_title;
use fr_page::text::text_lines::text_lines;

fn lines_of(html: &str) -> Vec<String> {
    text_lines(&parse_html(html))
}

#[test]
fn inline_tags_do_not_break_lines() {
    assert_eq!(lines_of("<p>Hello <b>world</b>!</p>"), vec!["Hello world!"]);
}

#[test]
fn block_tags_break_lines() {
    assert_eq!(lines_of("<div>one</div><div>two</div>"), vec!["one", "two"]);
}

#[test]
fn paragraphs_are_separated_by_a_gap() {
    assert_eq!(lines_of("<p>a</p><p>b</p>"), vec!["a", "", "b"]);
}

#[test]
fn whitespace_is_collapsed() {
    assert_eq!(lines_of("<p>a   \n   b</p>"), vec!["a b"]);
}

#[test]
fn scripts_and_styles_are_hidden() {
    assert_eq!(
        lines_of("<p>a</p><script>x</script><style>y</style>"),
        vec!["a"]
    );
}

#[test]
fn list_items_get_bullets() {
    assert_eq!(
        lines_of("<ul><li>One</li><li>Two</li></ul>"),
        vec!["\u{2022} One", "\u{2022} Two"]
    );
}

#[test]
fn line_breaks_split_lines() {
    assert_eq!(lines_of("<p>a<br>b</p>"), vec!["a", "b"]);
}

#[test]
fn an_empty_document_has_no_lines() {
    assert!(lines_of("").is_empty());
}

#[test]
fn a_non_breaking_space_is_kept() {
    assert_eq!(lines_of("<p>a&nbsp;b</p>"), vec!["a\u{a0}b"]);
}

#[test]
fn the_title_is_trimmed_and_collapsed() {
    let document = parse_html("<title>  Hi   there </title><p>x</p>");

    assert_eq!(document_title(&document), Some("Hi there".to_owned()));
}

#[test]
fn a_missing_title_is_none() {
    let document = parse_html("<p>x</p>");

    assert_eq!(document_title(&document), None);
}
