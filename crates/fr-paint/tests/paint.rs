mod support;

use fr_paint::display::display_item::DisplayItem;
use support::{paint, paint_with_image, solid_image};

const WHITE: (u8, u8, u8) = (255, 255, 255);
const RED: (u8, u8, u8) = (255, 0, 0);
const BLUE: (u8, u8, u8) = (0, 0, 255);

#[test]
fn a_background_is_painted_inside_its_box_only() {
    let page = paint(
        "<div style='background-color: #ff0000; height: 20px'></div>",
        "",
        0.0,
    );

    assert_eq!(page.rgb(50, 18), RED);
    assert_eq!(page.rgb(2, 2), WHITE);
    assert_eq!(page.rgb(50, 40), WHITE);
    assert_eq!(page.rgb(196, 18), WHITE);
}

#[test]
fn the_canvas_takes_the_body_background() {
    let page = paint("<p>x</p>", "body { background-color: #0000ff }", 0.0);

    assert_eq!(page.rgb(2, 100), BLUE);
}

#[test]
fn a_border_is_painted_around_the_box() {
    let page = paint(
        "<div style='width: 50px; height: 20px; border: 3px solid #0000ff'></div>",
        "",
        0.0,
    );

    assert_eq!(page.rgb(9, 20), BLUE);
    assert_eq!(page.rgb(30, 9), BLUE);
    assert_eq!(page.rgb(30, 32), BLUE);
    assert_eq!(page.rgb(62, 20), BLUE);
    assert_eq!(page.rgb(30, 20), WHITE);
}

#[test]
fn a_background_is_drawn_under_the_border_and_text() {
    let page = paint(
        "<div style='background-color: #ff0000; border: 2px solid #0000ff; height: 10px'></div>",
        "",
        0.0,
    );

    let colors: Vec<_> = page
        .list
        .items
        .iter()
        .filter_map(|item| match item {
            DisplayItem::Rect { color, .. } => Some((color.r, color.g, color.b)),
            _ => None,
        })
        .collect();
    assert_eq!(colors[0], RED);
    assert!(colors[1..].iter().all(|color| *color == BLUE));
}

#[test]
fn an_image_is_painted_at_its_position_and_size() {
    let page = paint_with_image(
        "<p style='margin: 0'><img src=x.png></p>",
        "",
        0.0,
        Some(solid_image(10, 10, [0, 255, 0, 255])),
    );

    assert_eq!(page.rgb(12, 12), (0, 255, 0));
    assert_eq!(page.rgb(30, 12), WHITE);
}

#[test]
fn an_image_is_scaled_to_the_requested_size() {
    let page = paint_with_image(
        "<p style='margin: 0'><img src=x.png width=40 height=20></p>",
        "",
        0.0,
        Some(solid_image(10, 10, [0, 255, 0, 255])),
    );

    assert_eq!(page.rgb(40, 20), (0, 255, 0));
    assert_eq!(page.rgb(60, 20), WHITE);
}

#[test]
fn a_half_transparent_image_blends_with_the_page() {
    let page = paint_with_image(
        "<p style='margin: 0'><img src=x.png></p>",
        "",
        0.0,
        Some(solid_image(10, 10, [127, 0, 0, 127])),
    );

    let (red, green, blue) = page.rgb(12, 12);
    assert!(red > 200);
    assert!(green > 100 && green < 160);
    assert!(blue > 100 && blue < 160);
}

#[test]
fn text_leaves_dark_pixels() {
    let page = paint("<p style='margin: 0'>Hello</p>", "", 0.0);

    let mut dark = 0;
    for y in 8..30 {
        for x in 8..80 {
            let (red, green, blue) = page.rgb(x, y);
            if red < 100 && green < 100 && blue < 100 {
                dark += 1;
            }
        }
    }
    assert!(dark > 20, "found only {dark} dark pixels");
}

#[test]
fn colored_text_keeps_its_color() {
    let page = paint(
        "<p style='margin: 0; color: #ff0000; font-size: 32px; font-weight: bold'>MMMM</p>",
        "",
        0.0,
    );

    let mut reddish = 0;
    for y in 8..50 {
        for x in 8..120 {
            let (red, green, blue) = page.rgb(x, y);
            if red > 200 && green < 80 && blue < 80 {
                reddish += 1;
            }
        }
    }
    assert!(reddish > 50, "found only {reddish} red pixels");
}

#[test]
fn scrolling_moves_the_content_up() {
    let page = paint(
        "<div style='background-color: #ff0000; height: 20px'></div>",
        "",
        8.0,
    );

    assert_eq!(page.rgb(50, 10), RED);
    assert_eq!(page.rgb(50, 25), WHITE);
}

#[test]
fn items_far_below_the_viewport_are_skipped_without_error() {
    let page = paint(
        "<div style='height: 5000px'></div><div style='background-color: #ff0000; height: 20px'></div>",
        "",
        0.0,
    );

    assert_eq!(page.rgb(50, 60), WHITE);
}
