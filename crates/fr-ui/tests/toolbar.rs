use fr_geom::geometry::point::Point;
use fr_ui::toolbar::hit_toolbar::hit_toolbar;
use fr_ui::toolbar::toolbar_metrics::{TOOLBAR_HEIGHT, address_rect, button_rect};
use fr_ui::toolbar::toolbar_target::ToolbarTarget;

fn center_of(target: ToolbarTarget) -> Point {
    let rect = button_rect(target).unwrap();
    Point::new(
        rect.origin.x + rect.size.width / 2.0,
        rect.origin.y + rect.size.height / 2.0,
    )
}

#[test]
fn each_button_is_found_at_its_center() {
    for target in [
        ToolbarTarget::Back,
        ToolbarTarget::Forward,
        ToolbarTarget::Reload,
    ] {
        assert_eq!(hit_toolbar(center_of(target), 800.0), Some(target));
    }
}

#[test]
fn the_address_field_is_found_inside_its_rectangle() {
    let rect = address_rect(800.0);
    let point = Point::new(rect.origin.x + 10.0, rect.origin.y + 5.0);

    assert_eq!(hit_toolbar(point, 800.0), Some(ToolbarTarget::Address));
}

#[test]
fn the_margin_beside_the_field_is_empty_toolbar() {
    let point = Point::new(795.0, 20.0);

    assert_eq!(hit_toolbar(point, 800.0), Some(ToolbarTarget::Empty));
}

#[test]
fn points_below_the_toolbar_are_not_on_it() {
    let point = Point::new(100.0, TOOLBAR_HEIGHT + 1.0);

    assert_eq!(hit_toolbar(point, 800.0), None);
}

#[test]
fn the_address_field_grows_with_the_window() {
    assert!(address_rect(1200.0).size.width > address_rect(800.0).size.width);
}

#[test]
fn a_tiny_window_gives_an_empty_address_field() {
    assert_eq!(address_rect(50.0).size.width, 0.0);
}

#[test]
fn the_address_field_has_no_button_rectangle() {
    assert!(button_rect(ToolbarTarget::Address).is_none());
}
