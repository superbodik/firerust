mod support;

use fr_layout::anchor::anchor_top::anchor_top;
use support::layout;

const PAGE: &str = "<p>intro</p>\
    <h2 id=first style='margin: 0; height: 30px'>First</h2>\
    <div style='height: 300px'>filler</div>\
    <h2 id='Second Part' style='margin: 0; height: 30px'>Second</h2>\
    <p><a name=legacy></a>old style anchor</p>\
    <div style='height: 80px'><span id=inside>inline target</span></div>";

#[test]
fn an_element_with_the_id_is_found_at_its_top_edge() {
    let page = layout(PAGE, "", 800.0);

    let top = anchor_top(&page.tree.root, &page.document, "first").expect("anchor");

    assert_eq!(top, page.boxed("first").border_box.origin.y);
}

#[test]
fn an_anchor_further_down_is_below_the_first_one() {
    let page = layout(PAGE, "", 800.0);

    let first = anchor_top(&page.tree.root, &page.document, "first").expect("first");
    let second = anchor_top(&page.tree.root, &page.document, "Second Part").expect("second");

    assert!(second > first + 300.0);
}

#[test]
fn an_empty_named_anchor_falls_back_to_the_box_around_it() {
    let page = layout(PAGE, "", 800.0);

    let top = anchor_top(&page.tree.root, &page.document, "legacy").expect("anchor");

    assert!(top > 300.0);
}

#[test]
fn an_inline_target_falls_back_to_its_block() {
    let page = layout(PAGE, "", 800.0);

    let top = anchor_top(&page.tree.root, &page.document, "inside").expect("anchor");

    assert!(top > 300.0);
}

#[test]
fn an_unknown_or_empty_fragment_is_nothing() {
    let page = layout(PAGE, "", 800.0);

    assert_eq!(anchor_top(&page.tree.root, &page.document, "missing"), None);
    assert_eq!(anchor_top(&page.tree.root, &page.document, ""), None);
}
