use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;
use fr_html::parse::parse_html::parse_html;

fn element_names(document: &Document) -> Vec<String> {
    document
        .walk(document.root())
        .filter_map(|edge| match edge {
            Edge::Enter(id) => document
                .node(id)?
                .kind()
                .as_element()
                .map(|element| element.name().to_string()),
            Edge::Leave(_) => None,
        })
        .collect()
}

fn all_text(document: &Document) -> String {
    document
        .walk(document.root())
        .filter_map(|edge| match edge {
            Edge::Enter(id) => document.node(id)?.kind().as_text().map(str::to_string),
            Edge::Leave(_) => None,
        })
        .collect()
}

#[test]
fn a_fragment_gets_the_full_document_skeleton() {
    let document = parse_html("<p>Hello <b>world</b></p>");

    assert_eq!(
        element_names(&document),
        vec!["html", "head", "body", "p", "b"]
    );
}

#[test]
fn text_is_kept_in_document_order() {
    let document = parse_html("<p>Hello <b>world</b>!</p>");

    assert_eq!(all_text(&document), "Hello world!");
}

#[test]
fn entities_are_decoded() {
    let document = parse_html("<p>Fish &amp; Chips &lt;3 &copy; 2026</p>");

    assert_eq!(all_text(&document), "Fish & Chips <3 \u{a9} 2026");
}

#[test]
fn attributes_are_kept() {
    let document = parse_html("<a href=\"/next\" id=\"go\">go</a>");

    let link = document
        .walk(document.root())
        .find_map(|edge| match edge {
            Edge::Enter(id) => document
                .node(id)?
                .kind()
                .as_element()
                .filter(|element| element.name() == "a"),
            Edge::Leave(_) => None,
        })
        .expect("link element");

    assert_eq!(link.attribute("href"), Some("/next"));
    assert_eq!(link.attribute("id"), Some("go"));
}

#[test]
fn comments_are_dropped() {
    let document = parse_html("<p>a<!-- hidden -->b</p>");

    assert_eq!(all_text(&document), "ab");
}

#[test]
fn unclosed_tags_are_repaired() {
    let document = parse_html("<div><p>one<p>two</div>");

    assert_eq!(
        element_names(&document),
        vec!["html", "head", "body", "div", "p", "p"]
    );
}

#[test]
fn cyrillic_text_survives() {
    let document = parse_html("<p>\u{41f}\u{440}\u{438}\u{432}\u{456}\u{442}</p>");

    assert_eq!(
        all_text(&document),
        "\u{41f}\u{440}\u{438}\u{432}\u{456}\u{442}"
    );
}

#[test]
fn very_deep_nesting_does_not_overflow_the_stack() {
    let html = format!("{}x{}", "<div>".repeat(5_000), "</div>".repeat(5_000));

    let document = parse_html(&html);

    assert!(document.node_count() > 5_000);
}
