mod support;

use fr_geom::geometry::point::Point;
use fr_layout::hit_test::element_at::element_at;
use support::layout;

#[test]
fn the_deepest_element_under_the_point_is_found() {
    let page = layout(
        "<div id=outer style='padding: 10px'><div id=inner style='height: 40px'></div></div>",
        "",
        800.0,
    );

    let inner = page.boxed("inner").border_box;
    let point = Point::new(inner.origin.x + 5.0, inner.origin.y + 5.0);
    assert_eq!(element_at(&page.tree.root, point), Some(page.id("inner")));
}

#[test]
fn the_padding_belongs_to_the_outer_element() {
    let page = layout(
        "<div id=outer style='padding: 10px'><div id=inner style='height: 40px'></div></div>",
        "",
        800.0,
    );

    let outer = page.boxed("outer").border_box;
    let point = Point::new(outer.origin.x + 2.0, outer.origin.y + 2.0);
    assert_eq!(element_at(&page.tree.root, point), Some(page.id("outer")));
}

#[test]
fn text_belongs_to_its_block() {
    let page = layout("<p id=a>hello there</p>", "", 800.0);

    let rect = page.boxed("a").border_box;
    let point = Point::new(rect.origin.x + 3.0, rect.origin.y + 3.0);
    assert_eq!(element_at(&page.tree.root, point), Some(page.id("a")));
}

#[test]
fn a_point_outside_every_box_finds_nothing() {
    let page = layout(
        "<div id=a style='height: 10px; width: 10px'></div>",
        "",
        800.0,
    );

    assert_eq!(element_at(&page.tree.root, Point::new(-5.0, -5.0)), None);
    assert_eq!(element_at(&page.tree.root, Point::new(790.0, 5000.0)), None);
}
