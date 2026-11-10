use fr_geom::geometry::point::Point;
use fr_ui::tabs::hit_tab_strip::hit_tab_strip;
use fr_ui::tabs::tab_strip_metrics::{
    TAB_STRIP_HEIGHT, close_rect, new_tab_rect, tab_rect, tab_width,
};
use fr_ui::tabs::tab_strip_target::TabStripTarget;

fn center(rect: fr_geom::geometry::rect::Rect) -> Point {
    Point::new(
        rect.origin.x + rect.size.width / 2.0,
        rect.origin.y + rect.size.height / 2.0,
    )
}

#[test]
fn a_single_tab_is_capped_at_the_maximum_width() {
    assert_eq!(tab_width(1, 1600.0), 220.0);
}

#[test]
fn many_tabs_share_the_strip() {
    let narrow = tab_width(8, 1000.0);

    assert!(narrow < 220.0);
    assert!(narrow >= 70.0);
}

#[test]
fn tabs_never_get_narrower_than_the_minimum() {
    assert_eq!(tab_width(50, 800.0), 70.0);
}

#[test]
fn tabs_sit_side_by_side() {
    let first = tab_rect(0, 3, 1000.0);
    let second = tab_rect(1, 3, 1000.0);

    assert_eq!(second.origin.x, first.right());
    assert_eq!(first.origin.y, second.origin.y);
}

#[test]
fn clicking_a_tab_selects_it() {
    let point = Point::new(
        tab_rect(1, 3, 1000.0).origin.x + 20.0,
        TAB_STRIP_HEIGHT / 2.0,
    );

    assert_eq!(
        hit_tab_strip(point, 3, 1000.0),
        Some(TabStripTarget::Tab(1))
    );
}

#[test]
fn clicking_the_cross_closes_the_tab() {
    let point = center(close_rect(2, 3, 1000.0));

    assert_eq!(
        hit_tab_strip(point, 3, 1000.0),
        Some(TabStripTarget::Close(2))
    );
}

#[test]
fn a_lone_tab_has_no_close_target() {
    let point = center(close_rect(0, 1, 1000.0));

    assert_eq!(
        hit_tab_strip(point, 1, 1000.0),
        Some(TabStripTarget::Tab(0))
    );
}

#[test]
fn the_plus_button_opens_a_new_tab() {
    let point = center(new_tab_rect(3, 1000.0));

    assert_eq!(hit_tab_strip(point, 3, 1000.0), Some(TabStripTarget::New));
}

#[test]
fn the_plus_button_follows_the_last_tab() {
    assert!(new_tab_rect(3, 1000.0).origin.x >= tab_rect(2, 3, 1000.0).right());
}

#[test]
fn empty_strip_space_is_not_a_page_click() {
    let point = Point::new(990.0, 10.0);

    assert_eq!(hit_tab_strip(point, 2, 1000.0), Some(TabStripTarget::Empty));
}

#[test]
fn below_the_strip_is_not_the_strip() {
    let point = Point::new(100.0, TAB_STRIP_HEIGHT + 1.0);

    assert_eq!(hit_tab_strip(point, 2, 1000.0), None);
}
