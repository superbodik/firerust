mod support;

use fr_geom::geometry::point::Point;
use fr_layout::hit_test::link_at::link_at;
use fr_text::engine::text_engine::TextEngine;
use support::{Laid, layout};

fn link_for(page: &Laid, paragraph: &str, dx: f32) -> Option<fr_dom::node::node_id::NodeId> {
    let rect = page.boxed(paragraph).border_box;
    let point = Point::new(rect.origin.x + dx, rect.origin.y + 8.0);
    link_at(
        &page.tree.root,
        &mut TextEngine::new(),
        &page.document,
        point,
    )
}

#[test]
fn text_inside_a_link_finds_the_anchor() {
    let page = layout(
        "<p id=p>Go <a id=l href='/x'>link text here</a> after</p>",
        "",
        800.0,
    );

    assert_eq!(link_for(&page, "p", 50.0), Some(page.id("l")));
}

#[test]
fn text_before_the_link_is_not_a_link() {
    let page = layout(
        "<p id=p>Go <a id=l href='/x'>link text here</a> after</p>",
        "",
        800.0,
    );

    assert_eq!(link_for(&page, "p", 2.0), None);
}

#[test]
fn text_after_the_link_is_not_a_link() {
    let page = layout(
        "<p id=p>Go <a id=l href='/x'>link</a> and a long tail of plain words</p>",
        "",
        800.0,
    );

    assert_eq!(link_for(&page, "p", 200.0), None);
}

#[test]
fn empty_space_right_of_the_text_is_not_a_link() {
    let page = layout("<p id=p><a id=l href='/x'>short</a></p>", "", 800.0);

    assert_eq!(link_for(&page, "p", 600.0), None);
}

#[test]
fn an_anchor_without_href_is_not_a_link() {
    let page = layout("<p id=p><a id=l name='top'>anchor text</a></p>", "", 800.0);

    assert_eq!(link_for(&page, "p", 30.0), None);
}

#[test]
fn formatting_inside_a_link_still_belongs_to_it() {
    let page = layout(
        "<p id=p><a id=l href='/x'><b>bold link</b></a></p>",
        "",
        800.0,
    );

    assert_eq!(link_for(&page, "p", 30.0), Some(page.id("l")));
}

#[test]
fn a_link_on_the_second_line_is_found() {
    let page = layout(
        "<p id=p>first line<br><a id=l href='/x'>second line link</a></p>",
        "",
        800.0,
    );

    let rect = page.boxed("p").border_box;
    let point = Point::new(rect.origin.x + 30.0, rect.origin.y + 24.0);
    assert_eq!(
        link_at(
            &page.tree.root,
            &mut TextEngine::new(),
            &page.document,
            point
        ),
        Some(page.id("l"))
    );
}

#[test]
fn a_click_on_a_non_text_box_is_not_a_link() {
    let page = layout("<div id=d style='height: 40px'></div>", "", 800.0);

    assert_eq!(link_for(&page, "d", 5.0), None);
}

fn link_at_box(
    page: &Laid,
    wanted: &str,
    dx: f32,
    dy: f32,
) -> Option<fr_dom::node::node_id::NodeId> {
    let rect = page.boxed(wanted).border_box;
    link_at(
        &page.tree.root,
        &mut TextEngine::new(),
        &page.document,
        Point::new(rect.origin.x + dx, rect.origin.y + dy),
    )
}

#[test]
fn the_empty_padding_of_a_block_link_is_part_of_the_link() {
    let page = layout(
        "<a id=l href='/x' style='display: block; padding: 20px; width: 200px'>Card</a>",
        "",
        800.0,
    );

    assert_eq!(link_at_box(&page, "l", 150.0, 5.0), Some(page.id("l")));
    assert_eq!(link_at_box(&page, "l", 190.0, 30.0), Some(page.id("l")));
}

#[test]
fn a_flex_row_that_is_a_link_is_clickable_across_its_whole_width() {
    let page = layout(
        "<a id=l class=row href='/x'><span>Left</span><span>Right</span></a>",
        "a.row { display: flex; justify-content: space-between; width: 400px }",
        800.0,
    );

    assert_eq!(link_at_box(&page, "l", 200.0, 6.0), Some(page.id("l")));
}

#[test]
fn an_image_inside_a_link_is_part_of_the_link() {
    let page = layout(
        "<a id=l href='/x'><div id=d style='width: 100px; height: 60px; background-color: #ccc'></div></a>",
        "",
        800.0,
    );

    assert_eq!(link_at_box(&page, "d", 50.0, 30.0), Some(page.id("l")));
}

#[test]
fn the_space_next_to_a_block_without_a_link_is_not_a_link() {
    let page = layout(
        "<div id=d style='width: 100px; height: 40px'>plain</div>",
        "",
        800.0,
    );

    assert_eq!(link_at_box(&page, "d", 50.0, 30.0), None);
}
