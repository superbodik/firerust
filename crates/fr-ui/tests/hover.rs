use fr_geom::geometry::point::Point;
use fr_geom::geometry::rect::Rect;
use fr_ui::hover::chrome_hover::ChromeHover;
use fr_ui::hover::hover_chrome::hover_chrome;
use fr_ui::tabs::tab_strip_metrics::{close_rect, new_tab_rect, tab_rect};
use fr_ui::toolbar::toolbar_metrics::{CHROME_HEIGHT, button_rect};
use fr_ui::toolbar::toolbar_target::ToolbarTarget;

fn center(rect: Rect) -> Point {
    Point::new(
        rect.origin.x + rect.size.width / 2.0,
        rect.origin.y + rect.size.height / 2.0,
    )
}

#[test]
fn a_tab_a_cross_and_the_plus_are_told_apart() {
    let width = 1000.0;

    assert_eq!(
        hover_chrome(
            Point::new(tab_rect(1, 3, width).origin.x + 20.0, 20.0),
            3,
            width
        ),
        ChromeHover::Tab(1)
    );
    assert_eq!(
        hover_chrome(center(close_rect(2, 3, width)), 3, width),
        ChromeHover::TabClose(2)
    );
    assert_eq!(
        hover_chrome(center(new_tab_rect(3, width)), 3, width),
        ChromeHover::NewTab
    );
}

#[test]
fn toolbar_buttons_are_reported_with_their_target() {
    let point = center(button_rect(ToolbarTarget::Reload, 1000.0).unwrap());

    assert_eq!(
        hover_chrome(point, 1, 1000.0),
        ChromeHover::Toolbar(ToolbarTarget::Reload)
    );
}

#[test]
fn empty_chrome_and_the_page_are_not_hovered() {
    assert_eq!(
        hover_chrome(Point::new(700.0, 10.0), 1, 1000.0),
        ChromeHover::Nothing
    );
    assert_eq!(
        hover_chrome(Point::new(100.0, CHROME_HEIGHT + 20.0), 1, 1000.0),
        ChromeHover::Nothing
    );
}

#[test]
fn the_window_buttons_are_hovered_at_the_right_end_of_the_strip() {
    use fr_ui::tabs::tab_strip_metrics::control_rect;
    use fr_ui::tabs::window_control::WindowControl;

    for control in WindowControl::ALL {
        assert_eq!(
            hover_chrome(center(control_rect(control, 1000.0)), 2, 1000.0),
            ChromeHover::Control(control)
        );
    }
}
