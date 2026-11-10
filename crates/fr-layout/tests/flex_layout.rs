mod support;

use fr_layout::boxes::box_kind::BoxKind;
use support::layout;

fn px(rect: fr_geom::geometry::rect::Rect) -> (f32, f32, f32, f32) {
    (
        rect.origin.x,
        rect.origin.y,
        rect.size.width,
        rect.size.height,
    )
}

#[test]
fn fixed_width_items_sit_in_a_row() {
    let page = layout(
        "<div id=c style='display: flex'><div id=a style='width: 100px; height: 20px'></div><div id=b style='width: 200px; height: 30px'></div></div>",
        "",
        800.0,
    );

    let first = px(page.boxed("a").border_box);
    let second = px(page.boxed("b").border_box);
    assert_eq!(first.0, 8.0);
    assert_eq!(second.0, 108.0);
    assert_eq!(first.1, second.1);
    assert_eq!(page.boxed("c").border_box.size.height, 30.0);
}

#[test]
fn flex_one_shares_the_free_space_equally() {
    let page = layout(
        "<div style='display: flex; width: 300px'><div id=a style='flex: 1; height: 10px'></div><div id=b style='flex: 1; height: 10px'></div></div>",
        "",
        800.0,
    );

    assert_eq!(page.boxed("a").border_box.size.width, 150.0);
    assert_eq!(page.boxed("b").border_box.size.width, 150.0);
    assert_eq!(page.boxed("b").border_box.origin.x, 158.0);
}

#[test]
fn grow_factors_split_the_space_in_proportion() {
    let page = layout(
        "<div style='display: flex; width: 300px'><div id=a style='flex: 1; height: 10px'></div><div id=b style='flex: 2; height: 10px'></div></div>",
        "",
        800.0,
    );

    assert_eq!(page.boxed("a").border_box.size.width, 100.0);
    assert_eq!(page.boxed("b").border_box.size.width, 200.0);
}

#[test]
fn a_fixed_item_keeps_its_width_beside_a_growing_one() {
    let page = layout(
        "<div style='display: flex; width: 400px'><div id=a style='width: 100px; height: 10px'></div><div id=b style='flex: 1; height: 10px'></div></div>",
        "",
        800.0,
    );

    assert_eq!(page.boxed("a").border_box.size.width, 100.0);
    assert_eq!(page.boxed("b").border_box.size.width, 300.0);
}

#[test]
fn gap_separates_the_items() {
    let page = layout(
        "<div style='display: flex; gap: 10px'><div id=a style='width: 50px; height: 10px'></div><div id=b style='width: 50px; height: 10px'></div></div>",
        "",
        800.0,
    );

    let first = page.boxed("a").border_box;
    assert_eq!(page.boxed("b").border_box.origin.x, first.right() + 10.0);
}

#[test]
fn justify_content_center_centers_the_items() {
    let page = layout(
        "<div style='display: flex; width: 300px; justify-content: center'><div id=a style='width: 100px; height: 10px'></div></div>",
        "",
        800.0,
    );

    assert_eq!(page.boxed("a").border_box.origin.x, 108.0);
}

#[test]
fn justify_content_space_between_pushes_items_apart() {
    let page = layout(
        "<div style='display: flex; width: 300px; justify-content: space-between'><div id=a style='width: 100px; height: 10px'></div><div id=b style='width: 100px; height: 10px'></div></div>",
        "",
        800.0,
    );

    assert_eq!(page.boxed("a").border_box.origin.x, 8.0);
    assert_eq!(page.boxed("b").border_box.right(), 308.0);
}

#[test]
fn justify_content_flex_end_packs_items_to_the_end() {
    let page = layout(
        "<div style='display: flex; width: 300px; justify-content: flex-end'><div id=a style='width: 100px; height: 10px'></div></div>",
        "",
        800.0,
    );

    assert_eq!(page.boxed("a").border_box.right(), 308.0);
}

#[test]
fn a_column_stacks_items_vertically() {
    let page = layout(
        "<div style='display: flex; flex-direction: column'><div id=a style='height: 20px'></div><div id=b style='height: 30px'></div></div>",
        "",
        800.0,
    );

    let first = page.boxed("a").border_box;
    let second = page.boxed("b").border_box;
    assert_eq!(second.origin.y, first.bottom());
    assert_eq!(first.origin.x, second.origin.x);
    assert_eq!(first.size.width, 784.0);
}

#[test]
fn wrapping_moves_items_to_the_next_line() {
    let page = layout(
        "<div style='display: flex; flex-wrap: wrap; width: 500px'><div id=a style='width: 300px; height: 20px'></div><div id=b style='width: 300px; height: 20px'></div></div>",
        "",
        800.0,
    );

    let first = page.boxed("a").border_box;
    let second = page.boxed("b").border_box;
    assert_eq!(second.origin.x, first.origin.x);
    assert_eq!(second.origin.y, first.bottom());
}

#[test]
fn without_wrapping_items_shrink_to_fit() {
    let page = layout(
        "<div style='display: flex; width: 300px'><div id=a style='width: 200px; height: 10px'></div><div id=b style='width: 200px; height: 10px'></div></div>",
        "",
        800.0,
    );

    assert_eq!(page.boxed("a").border_box.size.width, 150.0);
    assert_eq!(page.boxed("b").border_box.size.width, 150.0);
}

#[test]
fn items_stretch_to_the_tallest_by_default() {
    let page = layout(
        "<div style='display: flex'><div id=a style='width: 50px; height: 60px'></div><div id=b style='width: 50px'></div></div>",
        "",
        800.0,
    );

    assert_eq!(page.boxed("b").border_box.size.height, 60.0);
}

#[test]
fn align_items_center_centers_items_across() {
    let page = layout(
        "<div style='display: flex; align-items: center'><div id=a style='width: 50px; height: 20px'></div><div id=b style='width: 50px; height: 60px'></div></div>",
        "",
        800.0,
    );

    let top = page.boxed("b").border_box.origin.y;
    assert_eq!(page.boxed("a").border_box.origin.y, top + 20.0);
    assert_eq!(page.boxed("a").border_box.size.height, 20.0);
}

#[test]
fn text_items_shrink_to_their_content() {
    let page = layout(
        "<div style='display: flex'><div id=a>hi</div><div id=b>there</div></div>",
        "",
        800.0,
    );

    let first = page.boxed("a").border_box;
    let second = page.boxed("b").border_box;
    assert!(
        first.size.width < 60.0,
        "first item is {}",
        first.size.width
    );
    assert!(second.size.width < 100.0);
    assert!((second.origin.x - first.right()).abs() < 0.5);
}

#[test]
fn long_text_items_wrap_inside_their_share() {
    let page = layout(
        "<div style='display: flex; width: 300px'><div id=a style='flex: 1'>The quick brown fox jumps over the lazy dog and keeps running far away</div><div id=b style='flex: 1'>short</div></div>",
        "",
        800.0,
    );

    let tall = page.boxed("a").border_box;
    let short = page.boxed("b").border_box;
    assert_eq!(tall.size.width, short.size.width);
    assert!(tall.size.height > 40.0);
    assert_eq!(short.size.height, tall.size.height);
}

#[test]
fn loose_text_in_a_flex_container_becomes_an_item() {
    let page = layout(
        "<div id=c style='display: flex'>loose<div id=b style='width: 40px; height: 10px'></div></div>",
        "",
        800.0,
    );

    let container = page.boxed("c");
    assert_eq!(container.children.len(), 2);
    assert!(matches!(container.children[0].kind, BoxKind::Block));
    assert!(container.children[0].node.is_none());
}

#[test]
fn an_auto_left_margin_pushes_an_item_to_the_end() {
    let page = layout(
        "<div style='display: flex; width: 300px'><div id=a style='width: 100px; height: 10px; margin-left: auto'></div></div>",
        "",
        800.0,
    );

    assert_eq!(page.boxed("a").border_box.right(), 308.0);
}

#[test]
fn item_margins_are_kept() {
    let page = layout(
        "<div style='display: flex'><div id=a style='width: 50px; height: 10px; margin: 5px 7px'></div><div id=b style='width: 50px; height: 10px'></div></div>",
        "",
        800.0,
    );

    let first = page.boxed("a").border_box;
    assert_eq!(first.origin.x, 15.0);
    assert_eq!(first.origin.y, 13.0);
    assert_eq!(page.boxed("b").border_box.origin.x, first.right() + 7.0);
}

#[test]
fn padding_and_border_of_the_container_inset_the_items() {
    let page = layout(
        "<div id=c style='display: flex; padding: 10px; border: 2px solid red'><div id=a style='width: 50px; height: 10px'></div></div>",
        "",
        800.0,
    );

    assert_eq!(page.boxed("a").border_box.origin.x, 8.0 + 2.0 + 10.0);
    assert_eq!(page.boxed("c").border_box.size.height, 10.0 + 20.0 + 4.0);
}

#[test]
fn item_padding_and_border_count_inside_the_item() {
    let page = layout(
        "<div style='display: flex'><div id=a style='width: 100px; padding: 10px; border: 2px solid red; height: 10px'></div></div>",
        "",
        800.0,
    );

    assert_eq!(page.boxed("a").border_box.size.width, 124.0);
    assert_eq!(page.boxed("a").border_box.size.height, 34.0);
}

#[test]
fn nested_flex_containers_are_laid_out() {
    let page = layout(
        "<div style='display: flex; width: 400px'><div id=left style='flex: 1; display: flex; flex-direction: column'><div id=l1 style='height: 10px'></div><div id=l2 style='height: 10px'></div></div><div id=right style='width: 100px'></div></div>",
        "",
        800.0,
    );

    assert_eq!(page.boxed("left").border_box.size.width, 300.0);
    assert_eq!(page.boxed("l1").border_box.size.width, 300.0);
    assert_eq!(
        page.boxed("l2").border_box.origin.y,
        page.boxed("l1").border_box.bottom()
    );
    assert_eq!(page.boxed("right").border_box.size.height, 20.0);
}

#[test]
fn blocks_after_a_flex_container_continue_below_it() {
    let page = layout(
        "<div id=c style='display: flex'><div style='width: 50px; height: 40px'></div></div><div id=next style='height: 5px'></div>",
        "",
        800.0,
    );

    assert_eq!(
        page.boxed("next").border_box.origin.y,
        page.boxed("c").border_box.bottom()
    );
}

#[test]
fn an_empty_flex_container_has_no_height() {
    let page = layout("<div id=c style='display: flex'></div>", "", 800.0);

    assert_eq!(page.boxed("c").border_box.size.height, 0.0);
}

#[test]
fn short_words_in_a_row_never_wrap() {
    let page = layout(
        "<div style='display: flex; gap: 6px'><div id=a>About</div><div id=b>browser</div><div id=c>flexbox</div><div id=d>taffy</div></div><p id=p style='margin: 0'>About</p>",
        "",
        800.0,
    );

    let line = page.boxed("p").border_box.size.height;
    for id in ["a", "b", "c", "d"] {
        let height = page.boxed(id).border_box.size.height;
        assert!(
            (height - line).abs() < 1.0,
            "item {id} is {height}, a line is {line}"
        );
    }
}
