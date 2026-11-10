use fr_css::select::matches_selector::matches_selector;
use fr_css::select::parse_selector_list::parse_selector_list;
use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;
use fr_html::parse::parse_html::parse_html;

fn find(document: &Document, wanted: &str) -> NodeId {
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

fn matches(html: &str, selector: &str, wanted: &str) -> bool {
    let document = parse_html(html);
    let selectors = parse_selector_list(selector);
    let element = find(&document, wanted);
    selectors
        .iter()
        .any(|selector| matches_selector(&document, element, selector))
}

#[test]
fn an_attribute_presence_matches() {
    assert!(matches("<a id=x href='/y'>l</a>", "[href]", "x"));
    assert!(!matches("<a id=x>l</a>", "[href]", "x"));
}

#[test]
fn an_exact_value_matches() {
    assert!(matches("<input id=x type=text>", "[type=text]", "x"));
    assert!(matches("<input id=x type=text>", "[type='text']", "x"));
    assert!(matches("<input id=x type=text>", "[type=\"text\"]", "x"));
    assert!(!matches("<input id=x type=radio>", "[type=text]", "x"));
}

#[test]
fn a_tag_and_an_attribute_combine() {
    assert!(matches("<input id=x type=text>", "input[type=text]", "x"));
    assert!(!matches(
        "<button id=x type=text>b</button>",
        "input[type=text]",
        "x"
    ));
}

#[test]
fn a_class_and_an_attribute_combine() {
    assert!(matches(
        "<a id=x class=nav href='/z'>l</a>",
        "a.nav[href]",
        "x"
    ));
}

#[test]
fn several_attributes_must_all_match() {
    assert!(matches(
        "<input id=x type=text name=user>",
        "[type=text][name=user]",
        "x"
    ));
    assert!(!matches(
        "<input id=x type=text name=other>",
        "[type=text][name=user]",
        "x"
    ));
}

#[test]
fn a_word_list_matches_one_word() {
    assert!(matches("<p id=x rel='a b c'>t</p>", "[rel~=b]", "x"));
    assert!(!matches("<p id=x rel='ab c'>t</p>", "[rel~=b]", "x"));
}

#[test]
fn a_dash_match_accepts_the_value_or_a_prefix_with_a_dash() {
    assert!(matches("<p id=x lang=en>t</p>", "[lang|=en]", "x"));
    assert!(matches("<p id=x lang=en-US>t</p>", "[lang|=en]", "x"));
    assert!(!matches("<p id=x lang=english>t</p>", "[lang|=en]", "x"));
}

#[test]
fn prefix_suffix_and_substring_matches_work() {
    assert!(matches(
        "<a id=x href='https://a.test/'>l</a>",
        "[href^=https]",
        "x"
    ));
    assert!(matches(
        "<a id=x href='file.pdf'>l</a>",
        "[href$='.pdf']",
        "x"
    ));
    assert!(matches("<a id=x href='/a/b/c'>l</a>", "[href*='/b/']", "x"));
    assert!(!matches("<a id=x href='/a'>l</a>", "[href^=https]", "x"));
}

#[test]
fn empty_values_never_match_the_partial_operators() {
    assert!(!matches("<a id=x href='abc'>l</a>", "[href^='']", "x"));
    assert!(!matches("<a id=x href='abc'>l</a>", "[href*='']", "x"));
}

#[test]
fn the_case_flag_ignores_case() {
    assert!(matches("<input id=x type=TEXT>", "[type=text i]", "x"));
    assert!(!matches("<input id=x type=TEXT>", "[type=text]", "x"));
}

#[test]
fn attribute_names_are_case_insensitive_in_the_selector() {
    assert!(matches("<a id=x href='/y'>l</a>", "[HREF]", "x"));
}

#[test]
fn a_descendant_chain_can_use_attributes() {
    assert!(matches(
        "<form id=f action='/s'><p><input id=x name=q></p></form>",
        "form[action] input[name=q]",
        "x"
    ));
}

#[test]
fn a_broken_attribute_selector_matches_nothing() {
    assert!(parse_selector_list("[=x]").is_empty());
    assert!(parse_selector_list("[type=]").is_empty());
}

#[test]
fn attributes_raise_the_specificity_like_classes() {
    let plain = parse_selector_list("input");
    let with_attribute = parse_selector_list("input[type=text]");
    let with_class = parse_selector_list("input.x");

    assert!(with_attribute[0].specificity() > plain[0].specificity());
    assert_eq!(with_attribute[0].specificity(), with_class[0].specificity());
}
