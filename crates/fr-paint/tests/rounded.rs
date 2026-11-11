mod support;

use fr_paint::display::display_item::DisplayItem;
use support::{paint, paint_scaled};

const WHITE: (u8, u8, u8) = (255, 255, 255);
const RED: (u8, u8, u8) = (255, 0, 0);
const BLUE: (u8, u8, u8) = (0, 0, 255);

const BOX: &str =
    "<div style='width: 60px; height: 40px; background-color: #ff0000; border-radius: 12px'></div>";

#[test]
fn a_rounded_background_cuts_its_corners_and_keeps_the_middle() {
    let page = paint(BOX, "", 0.0);

    assert_eq!(page.rgb(8, 8), WHITE);
    assert_eq!(page.rgb(67, 8), WHITE);
    assert_eq!(page.rgb(8, 47), WHITE);
    assert_eq!(page.rgb(67, 47), WHITE);
    assert_eq!(page.rgb(38, 28), RED);
    assert_eq!(page.rgb(38, 9), RED);
    assert_eq!(page.rgb(9, 28), RED);
}

#[test]
fn a_box_without_a_radius_keeps_the_plain_rectangle_item() {
    let page = paint(
        "<div style='width: 60px; height: 40px; background-color: #ff0000'></div>",
        "",
        0.0,
    );

    assert!(
        page.list
            .items
            .iter()
            .all(|item| !matches!(item, DisplayItem::RoundedRect { .. }))
    );
    assert_eq!(page.rgb(8, 8), RED);
}

#[test]
fn a_rounded_border_is_a_ring_with_clear_corners_and_a_clear_middle() {
    let page = paint(
        "<div style='width: 60px; height: 40px; border: 3px solid #0000ff; border-radius: 14px'></div>",
        "",
        0.0,
    );

    assert_eq!(page.rgb(8, 8), WHITE);
    assert_eq!(page.rgb(40, 9), BLUE);
    assert_eq!(page.rgb(9, 30), BLUE);
    assert_eq!(page.rgb(40, 30), WHITE);
    assert!(
        page.list
            .items
            .iter()
            .any(|item| matches!(item, DisplayItem::Border { .. }))
    );
}

#[test]
fn a_radius_of_half_the_side_makes_a_circle() {
    let page = paint(
        "<div style='width: 40px; height: 40px; background-color: #ff0000; border-radius: 50%'></div>",
        "",
        0.0,
    );

    assert_eq!(page.rgb(8, 8), WHITE);
    assert_eq!(page.rgb(47, 47), WHITE);
    assert_eq!(page.rgb(28, 28), RED);
    assert_eq!(page.rgb(28, 9), RED);
}

#[test]
fn only_the_named_corners_are_rounded() {
    let page = paint(
        "<div style='width: 60px; height: 40px; background-color: #ff0000; border-top-left-radius: 16px'></div>",
        "",
        0.0,
    );

    assert_eq!(page.rgb(8, 8), WHITE);
    assert_eq!(page.rgb(67, 8), RED);
    assert_eq!(page.rgb(8, 47), RED);
}

#[test]
fn the_radius_grows_with_the_page_zoom() {
    let page = paint_scaled(BOX, "", 0.0, 2.0);

    assert_eq!(page.rgb(16, 16), WHITE);
    assert_eq!(page.rgb(76, 56), RED);
}
