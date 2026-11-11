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

const LIST: &str =
    "<ul><li id=a>one</li><li id=b>two</li><li id=c>three</li><li id=d>four</li></ul>";

#[test]
fn first_and_last_child_pick_the_ends() {
    assert!(matches(LIST, "li:first-child", "a"));
    assert!(!matches(LIST, "li:first-child", "b"));
    assert!(matches(LIST, "li:last-child", "d"));
    assert!(!matches(LIST, "li:last-child", "c"));
}

#[test]
fn only_child_needs_a_single_element() {
    assert!(matches("<div><p id=x>t</p></div>", "p:only-child", "x"));
    assert!(!matches(LIST, "li:only-child", "a"));
}

#[test]
fn nth_child_understands_numbers_and_formulas() {
    assert!(matches(LIST, "li:nth-child(2)", "b"));
    assert!(!matches(LIST, "li:nth-child(2)", "a"));
    assert!(matches(LIST, "li:nth-child(odd)", "a"));
    assert!(matches(LIST, "li:nth-child(odd)", "c"));
    assert!(!matches(LIST, "li:nth-child(odd)", "b"));
    assert!(matches(LIST, "li:nth-child(even)", "d"));
    assert!(matches(LIST, "li:nth-child(2n+1)", "c"));
    assert!(matches(LIST, "li:nth-child(n+3)", "d"));
    assert!(!matches(LIST, "li:nth-child(n+3)", "b"));
    assert!(matches(LIST, "li:nth-child(-n+2)", "a"));
    assert!(!matches(LIST, "li:nth-child(-n+2)", "c"));
}

#[test]
fn nth_last_child_counts_from_the_end() {
    assert!(matches(LIST, "li:nth-last-child(1)", "d"));
    assert!(matches(LIST, "li:nth-last-child(2)", "c"));
    assert!(!matches(LIST, "li:nth-last-child(2)", "d"));
}

#[test]
fn text_between_elements_is_not_counted() {
    let html = "<div>text<span id=a>x</span> more <span id=b>y</span></div>";

    assert!(matches(html, "span:first-child", "a"));
    assert!(matches(html, "span:nth-child(2)", "b"));
}

#[test]
fn not_excludes_what_it_names() {
    assert!(matches("<p id=x class=plain>t</p>", "p:not(.fancy)", "x"));
    assert!(!matches("<p id=x class=fancy>t</p>", "p:not(.fancy)", "x"));
    assert!(matches("<a id=x href=/y>l</a>", "a:not([target])", "x"));
    assert!(!matches(
        "<a id=x href=/y target=_blank>l</a>",
        "a:not([target])",
        "x"
    ));
    assert!(matches("<p id=x class=c>t</p>", "p:not(.a, .b)", "x"));
    assert!(!matches("<p id=x class=b>t</p>", "p:not(.a, .b)", "x"));
}

#[test]
fn link_matches_anchors_with_an_href_only() {
    assert!(matches("<a id=x href=/y>l</a>", "a:link", "x"));
    assert!(matches("<a id=x href=/y>l</a>", ":any-link", "x"));
    assert!(!matches("<a id=x name=top>l</a>", "a:link", "x"));
}

#[test]
fn root_is_the_html_element_only() {
    let html = "<html id=r><body><p id=x>t</p></body></html>";

    assert!(matches(html, ":root", "r"));
    assert!(!matches(html, ":root", "x"));
}

#[test]
fn empty_and_checked_look_at_the_element() {
    assert!(matches("<div><p id=x></p></div>", "p:empty", "x"));
    assert!(!matches("<div><p id=x>t</p></div>", "p:empty", "x"));
    assert!(matches(
        "<input id=x type=checkbox checked>",
        "input:checked",
        "x"
    ));
    assert!(!matches("<input id=x type=checkbox>", "input:checked", "x"));
}

#[test]
fn the_next_sibling_combinator_needs_the_element_right_before() {
    assert!(matches(LIST, "li + li", "b"));
    assert!(!matches(LIST, "li + li", "a"));
    assert!(matches("<h2>t</h2><p id=x>t</p>", "h2 + p", "x"));
    assert!(!matches(
        "<h2>t</h2><div></div><p id=x>t</p>",
        "h2 + p",
        "x"
    ));
}

#[test]
fn the_subsequent_sibling_combinator_looks_at_every_earlier_sibling() {
    assert!(matches("<h2>t</h2><div></div><p id=x>t</p>", "h2 ~ p", "x"));
    assert!(!matches("<p id=x>t</p><h2>t</h2>", "h2 ~ p", "x"));
}

#[test]
fn dynamic_pseudo_classes_never_match_and_do_not_break_the_list() {
    assert!(!matches("<a id=x href=/y>l</a>", "a:hover", "x"));
    assert!(matches("<a id=x href=/y>l</a>", "a:hover, a", "x"));
    assert!(!matches("<a id=x href=/y>l</a>", "a::before", "x"));
}

#[test]
fn a_pseudo_class_raises_the_specificity_like_a_class() {
    let plain = parse_selector_list("li");
    let pseudo = parse_selector_list("li:first-child");
    let class = parse_selector_list("li.x");

    assert!(pseudo[0].specificity() > plain[0].specificity());
    assert_eq!(pseudo[0].specificity(), class[0].specificity());
}
