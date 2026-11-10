mod support;

use fr_dom::node::node_id::NodeId;
use fr_geom::geometry::point::Point;
use fr_layout::hit_test::event_target_at::event_target_at;
use fr_layout::hit_test::run_at::run_at;
use fr_text::engine::text_engine::TextEngine;
use support::{Laid, layout};

fn target_for(page: &Laid, block: &str, dx: f32, dy: f32) -> Option<NodeId> {
    let rect = page.boxed(block).border_box;
    let point = Point::new(rect.origin.x + dx, rect.origin.y + dy);
    event_target_at(&page.tree.root, &mut TextEngine::new(), point)
}

#[test]
fn text_in_an_inline_element_targets_that_element() {
    let page = layout(
        "<p id=p>Go <span id=s>over here please</span> after</p>",
        "",
        800.0,
    );

    assert_eq!(target_for(&page, "p", 60.0, 8.0), Some(page.id("s")));
}

#[test]
fn text_directly_in_a_block_targets_the_block() {
    let page = layout("<p id=p>plain words only</p>", "", 800.0);

    assert_eq!(target_for(&page, "p", 20.0, 8.0), Some(page.id("p")));
}

#[test]
fn nested_inline_elements_target_the_innermost() {
    let page = layout(
        "<p id=p><span id=outer><b id=inner>deep text here</b></span></p>",
        "",
        800.0,
    );

    assert_eq!(target_for(&page, "p", 30.0, 8.0), Some(page.id("inner")));
}

#[test]
fn an_empty_block_targets_itself() {
    let page = layout("<div id=d style='height: 40px'></div>", "", 800.0);

    assert_eq!(target_for(&page, "d", 5.0, 5.0), Some(page.id("d")));
}

#[test]
fn space_right_of_the_text_targets_the_block() {
    let page = layout("<p id=p><span id=s>short</span></p>", "", 800.0);

    assert_eq!(target_for(&page, "p", 600.0, 8.0), Some(page.id("p")));
}

#[test]
fn run_at_ignores_boxes_that_are_not_text() {
    let page = layout("<div id=d style='height: 40px'></div>", "", 800.0);
    let rect = page.boxed("d").border_box;

    let found = run_at(
        &page.tree.root,
        &mut TextEngine::new(),
        Point::new(rect.origin.x + 5.0, rect.origin.y + 5.0),
    );

    assert_eq!(found, None);
}

#[test]
fn an_image_alt_text_targets_the_image() {
    let page = layout("<p id=p><img id=i alt='picture of a cat'></p>", "", 800.0);

    assert_eq!(target_for(&page, "p", 30.0, 8.0), Some(page.id("i")));
}
