mod support;

use fr_geom::color::rgba::Rgba;
use fr_layout::boxes::box_kind::BoxKind;
use support::layout;

const EMPTY_BLOCK: &str = "<div id=a style='height: 10px'></div>";

#[test]
fn a_block_fills_the_width_inside_the_body_margin() {
    let page = layout(EMPTY_BLOCK, "", 800.0);

    let rect = page.boxed("a").border_box;
    assert_eq!(rect.origin.x, 8.0);
    assert_eq!(rect.size.width, 784.0);
    assert_eq!(rect.size.height, 10.0);
}

#[test]
fn auto_margins_center_a_fixed_width_box() {
    let page = layout(
        "<div id=a style='width: 200px; margin: 0 auto; height: 5px'></div>",
        "",
        800.0,
    );

    let rect = page.boxed("a").border_box;
    assert_eq!(rect.size.width, 200.0);
    assert_eq!(rect.origin.x, 300.0);
}

#[test]
fn max_width_with_auto_margins_centers_the_box() {
    let page = layout(
        "<div id=a style='max-width: 600px; margin: 0 auto; height: 5px'></div>",
        "",
        1000.0,
    );

    let rect = page.boxed("a").border_box;
    assert_eq!(rect.size.width, 600.0);
    assert_eq!(rect.origin.x, 200.0);
}

#[test]
fn a_narrow_viewport_keeps_max_width_from_widening_the_box() {
    let page = layout(
        "<div id=a style='max-width: 600px; margin: 0 auto; height: 5px'></div>",
        "",
        400.0,
    );

    assert_eq!(page.boxed("a").border_box.size.width, 384.0);
}

#[test]
fn padding_and_border_grow_the_box() {
    let page = layout(
        "<div id=a style='width: 100px; padding: 10px; border: 2px solid #ff0000; height: 20px'></div>",
        "",
        800.0,
    );

    let layout_box = page.boxed("a");
    assert_eq!(layout_box.border_box.size.width, 124.0);
    assert_eq!(layout_box.border_box.size.height, 44.0);
    assert_eq!(layout_box.border_width.left, 2.0);
    assert_eq!(layout_box.border_color, Rgba::opaque(255, 0, 0));
}

#[test]
fn border_box_sizing_keeps_the_total_width() {
    let page = layout(
        "<div id=a style='box-sizing: border-box; width: 100px; padding: 10px; border: 2px solid red; height: 50px'></div>",
        "",
        800.0,
    );

    let rect = page.boxed("a").border_box;
    assert_eq!(rect.size.width, 100.0);
    assert_eq!(rect.size.height, 50.0);
}

#[test]
fn percent_widths_use_the_container() {
    let page = layout(
        "<div id=a style='width: 50%; height: 5px'></div>",
        "",
        800.0,
    );

    assert_eq!(page.boxed("a").border_box.size.width, 392.0);
}

#[test]
fn min_width_wins_over_a_smaller_width() {
    let page = layout(
        "<div id=a style='width: 50px; min-width: 120px; height: 5px'></div>",
        "",
        800.0,
    );

    assert_eq!(page.boxed("a").border_box.size.width, 120.0);
}

#[test]
fn sibling_margins_collapse_to_the_larger_one() {
    let page = layout(
        "<div id=a style='height: 50px; margin-bottom: 30px'></div><div id=b style='height: 50px; margin-top: 10px'></div>",
        "",
        800.0,
    );

    let first = page.boxed("a").border_box;
    let second = page.boxed("b").border_box;
    assert_eq!(first.origin.y, 8.0);
    assert_eq!(second.origin.y, first.bottom() + 30.0);
}

#[test]
fn a_parent_and_its_first_child_share_their_top_margin() {
    let page = layout(
        "<div id=outer style='margin-top: 20px'><div id=inner style='margin-top: 30px; height: 10px'></div></div>",
        "",
        800.0,
    );

    assert_eq!(page.boxed("outer").border_box.origin.y, 30.0);
    assert_eq!(page.boxed("inner").border_box.origin.y, 30.0);
}

#[test]
fn padding_stops_margins_from_collapsing_through() {
    let page = layout(
        "<div id=outer style='margin-top: 20px; padding-top: 1px'><div id=inner style='margin-top: 30px; height: 10px'></div></div>",
        "",
        800.0,
    );

    assert_eq!(page.boxed("outer").border_box.origin.y, 20.0);
    assert_eq!(page.boxed("inner").border_box.origin.y, 51.0);
}

#[test]
fn a_last_child_margin_leaks_out_of_its_parent() {
    let page = layout(
        "<div id=outer><div style='height: 10px; margin-bottom: 40px'></div></div><div id=next style='height: 5px'></div>",
        "",
        800.0,
    );

    let outer = page.boxed("outer").border_box;
    assert_eq!(outer.size.height, 10.0);
    assert_eq!(
        page.boxed("next").border_box.origin.y,
        outer.bottom() + 40.0
    );
}

#[test]
fn an_element_with_display_none_has_no_box() {
    let page = layout(
        "<div id=a style='display: none; height: 10px'></div>",
        "",
        800.0,
    );

    assert!(page.find("a").is_none());
}

#[test]
fn nested_padding_moves_the_content() {
    let page = layout(
        "<div style='padding-left: 40px'><div id=inner style='height: 5px'></div></div>",
        "",
        800.0,
    );

    assert_eq!(page.boxed("inner").border_box.origin.x, 48.0);
}

#[test]
fn list_items_are_indented_and_get_a_marker() {
    let page = layout("<ul><li id=a>one</li></ul>", "", 800.0);

    let item = page.boxed("a");
    assert_eq!(item.border_box.origin.x, 48.0);
    let marker = item
        .children
        .iter()
        .find(|child| matches!(child.kind, BoxKind::Marker(_)))
        .expect("marker");
    assert!(marker.border_box.right() <= item.border_box.origin.x);
}

#[test]
fn the_body_background_is_the_canvas_background() {
    let page = layout("<p>x</p>", "body { background-color: #112233 }", 800.0);

    assert_eq!(page.tree.background, Rgba::opaque(0x11, 0x22, 0x33));
}

#[test]
fn a_block_background_is_kept_on_its_box() {
    let page = layout(
        "<div id=a style='background-color: #ffcc00; height: 5px'></div>",
        "",
        800.0,
    );

    assert_eq!(page.boxed("a").background, Rgba::opaque(255, 204, 0));
}

#[test]
fn the_page_height_covers_the_content() {
    let page = layout(EMPTY_BLOCK, "", 800.0);

    assert_eq!(page.tree.height, 8.0 + 10.0 + 8.0);
}

#[test]
fn an_empty_document_has_no_height() {
    let page = layout("", "", 800.0);

    assert!(page.tree.height <= 16.0);
}
