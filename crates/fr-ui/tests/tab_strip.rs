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
    assert_eq!(tab_width(1, 1600.0), 240.0);
}

#[test]
fn many_tabs_share_the_strip() {
    let narrow = tab_width(8, 1000.0);

    assert!(narrow < 240.0);
    assert!(narrow >= 80.0);
}

#[test]
fn tabs_never_get_narrower_than_the_minimum() {
    assert_eq!(tab_width(50, 800.0), 80.0);
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
    let point = Point::new(700.0, 10.0);

    assert_eq!(hit_tab_strip(point, 2, 1000.0), Some(TabStripTarget::Empty));
}

#[test]
fn below_the_strip_is_not_the_strip() {
    let point = Point::new(100.0, TAB_STRIP_HEIGHT + 1.0);

    assert_eq!(hit_tab_strip(point, 2, 1000.0), None);
}

#[test]
fn the_window_buttons_fill_the_right_end_in_order() {
    use fr_ui::tabs::tab_strip_metrics::{CONTROL_WIDTH, control_rect, controls_width};
    use fr_ui::tabs::window_control::WindowControl;

    let minimize = control_rect(WindowControl::Minimize, 1000.0);
    let maximize = control_rect(WindowControl::Maximize, 1000.0);
    let close = control_rect(WindowControl::Close, 1000.0);

    assert_eq!(close.right(), 1000.0);
    assert_eq!(maximize.right(), close.origin.x);
    assert_eq!(minimize.right(), maximize.origin.x);
    assert_eq!(minimize.size.width, CONTROL_WIDTH);
    assert_eq!(1000.0 - minimize.origin.x, controls_width());
    assert_eq!(close.size.height, TAB_STRIP_HEIGHT);
}

#[test]
fn clicking_a_window_button_reports_it() {
    use fr_ui::tabs::tab_strip_metrics::control_rect;
    use fr_ui::tabs::window_control::WindowControl;

    for control in WindowControl::ALL {
        let point = center(control_rect(control, 1000.0));

        assert_eq!(
            hit_tab_strip(point, 3, 1000.0),
            Some(TabStripTarget::Control(control))
        );
    }
}

#[test]
fn tabs_and_the_plus_never_run_under_the_window_buttons() {
    use fr_ui::tabs::tab_strip_metrics::control_rect;
    use fr_ui::tabs::window_control::WindowControl;

    let controls_left = control_rect(WindowControl::Minimize, 800.0).origin.x;

    assert!(new_tab_rect(6, 800.0).right() <= controls_left);
    assert!(tab_rect(5, 6, 800.0).right() <= controls_left);
}

#[test]
fn there_is_always_some_strip_left_to_drag() {
    use fr_ui::tabs::tab_strip_metrics::control_rect;
    use fr_ui::tabs::window_control::WindowControl;

    let controls_left = control_rect(WindowControl::Minimize, 1000.0).origin.x;
    let plus = new_tab_rect(8, 1000.0);

    assert!(controls_left - plus.right() >= 30.0);
}
