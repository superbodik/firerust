use fr_css::select::matches_selector::matches_selector;
use fr_css::select::parse_selector_list::parse_selector_list;
use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;
use fr_html::parse::parse_html::parse_html;

fn find_by_id(document: &Document, wanted: &str) -> NodeId {
    document
        .walk(document.root())
        .find_map(|edge| match edge {
            Edge::Enter(id) => {
                let element = document.node(id)?.kind().as_element()?;
                (element.attribute("id") == Some(wanted)).then_some(id)
            }
            Edge::Leave(_) => None,
        })
        .expect("element with the id")
}

fn matches(html: &str, id: &str, selector: &str) -> bool {
    let document = parse_html(html);
    let element = find_by_id(&document, id);
    parse_selector_list(selector)
        .iter()
        .any(|parsed| matches_selector(&document, element, parsed))
}

#[test]
fn a_tag_selector_matches_by_name() {
    assert!(matches("<p id=x>a</p>", "x", "p"));
    assert!(!matches("<p id=x>a</p>", "x", "div"));
}

#[test]
fn a_tag_selector_ignores_case() {
    assert!(matches("<p id=x>a</p>", "x", "P"));
}

#[test]
fn a_class_selector_matches_one_of_several_classes() {
    assert!(matches("<p id=x class='a b c'>t</p>", "x", ".b"));
    assert!(!matches("<p id=x class='a b c'>t</p>", "x", ".d"));
}

#[test]
fn a_compound_needs_every_part() {
    assert!(matches("<p id=x class='note big'>t</p>", "x", "p.note.big"));
    assert!(!matches("<p id=x class='note'>t</p>", "x", "p.note.big"));
}

#[test]
fn an_id_selector_matches_the_id() {
    assert!(matches("<p id=x>t</p>", "x", "#x"));
    assert!(!matches("<p id=x>t</p>", "x", "#y"));
}

#[test]
fn the_universal_selector_matches_everything() {
    assert!(matches("<p id=x>t</p>", "x", "*"));
}

#[test]
fn a_descendant_selector_matches_any_ancestor() {
    let html = "<div class=outer><section><p id=x>t</p></section></div>";

    assert!(matches(html, "x", ".outer p"));
    assert!(matches(html, "x", "div section p"));
    assert!(!matches(html, "x", "ul p"));
}

#[test]
fn a_child_selector_needs_the_direct_parent() {
    let html = "<div class=outer><section><p id=x>t</p></section></div>";

    assert!(matches(html, "x", "section > p"));
    assert!(!matches(html, "x", ".outer > p"));
}

#[test]
fn a_child_selector_works_without_spaces() {
    assert!(matches("<div><p id=x>t</p></div>", "x", "div>p"));
}

#[test]
fn a_descendant_chain_can_backtrack() {
    let html = "<div class=a><div class=b><p id=x>t</p></div></div>";

    assert!(matches(html, "x", ".a .b p"));
    assert!(matches(html, "x", ".a p"));
}

#[test]
fn a_selector_list_matches_when_any_selector_does() {
    assert!(matches("<p id=x>t</p>", "x", "h1, p"));
}

#[test]
fn specificity_counts_ids_classes_and_tags() {
    let specificity = |text: &str| parse_selector_list(text)[0].specificity();

    assert!(specificity("#a") > specificity(".a.b.c"));
    assert!(specificity(".a") > specificity("div p span"));
    assert!(specificity("div p") > specificity("p"));
    assert_eq!(specificity("*"), 0);
}

#[test]
fn unsupported_or_broken_selectors_give_nothing() {
    assert!(parse_selector_list("a:hover").is_empty());
    assert!(parse_selector_list("a[href]").is_empty());
    assert!(parse_selector_list("> p").is_empty());
    assert!(parse_selector_list("p >").is_empty());
    assert!(parse_selector_list(".").is_empty());
    assert!(parse_selector_list("").is_empty());
}
