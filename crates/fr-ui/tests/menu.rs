use fr_geom::geometry::point::Point;
use fr_geom::geometry::rect::Rect;
use fr_ui::menu::entry_rects::entry_rects;
use fr_ui::menu::hit_menu::hit_menu;
use fr_ui::menu::menu_entry::MenuEntry;
use fr_ui::menu::menu_hit::MenuHit;
use fr_ui::menu::menu_kind::MenuKind;
use fr_ui::menu::menu_metrics::{menu_height, menu_rect};
use fr_ui::menu::menu_target::MenuTarget;
use fr_ui::menu::start_button_rect::start_button_rect;
use fr_ui::menu::zoom_in_rect::zoom_in_rect;
use fr_ui::menu::zoom_out_rect::zoom_out_rect;
use fr_ui::menu::zoom_value_rect::zoom_value_rect;

const WIDTH: f32 = 960.0;
const HEIGHT: f32 = 600.0;

fn centre(rect: Rect) -> Point {
    Point::new(
        rect.origin.x + rect.size.width / 2.0,
        rect.origin.y + rect.size.height / 2.0,
    )
}

fn row_of(kind: MenuKind, wanted: MenuEntry) -> Rect {
    entry_rects(kind, WIDTH, HEIGHT)
        .into_iter()
        .find(|(entry, _)| *entry == wanted)
        .map(|(_, rect)| rect)
        .expect("row")
}

#[test]
fn the_main_menu_hangs_from_the_right_edge_under_the_toolbar() {
    let popup = menu_rect(MenuKind::Main, WIDTH, HEIGHT);

    assert!(popup.right() < WIDTH);
    assert!(popup.right() > WIDTH - 30.0);
    assert!(popup.origin.y > 70.0 && popup.origin.y < 90.0);
    assert!(popup.bottom() < HEIGHT);
}

#[test]
fn the_start_menu_sits_above_the_corner_button() {
    let popup = menu_rect(MenuKind::Start, WIDTH, HEIGHT);
    let button = start_button_rect(HEIGHT);

    assert!(popup.bottom() < button.origin.y);
    assert!(popup.origin.x <= button.origin.x + 1.0);
    assert!(button.bottom() < HEIGHT);
}

#[test]
fn the_rows_are_stacked_without_overlap_inside_the_popup() {
    let popup = menu_rect(MenuKind::Main, WIDTH, HEIGHT);
    let rows = entry_rects(MenuKind::Main, WIDTH, HEIGHT);

    for pair in rows.windows(2) {
        assert!(pair[0].1.bottom() <= pair[1].1.origin.y + 0.01);
    }
    let last = rows.last().expect("rows").1;
    assert!(last.bottom() <= popup.bottom());
    assert!((menu_height(MenuKind::Main) - popup.size.height).abs() < 0.01);
}

#[test]
fn a_click_on_a_row_names_its_target() {
    let row = row_of(MenuKind::Main, MenuEntry::Item(MenuTarget::Bookmarks));

    assert_eq!(
        hit_menu(MenuKind::Main, centre(row), WIDTH, HEIGHT),
        MenuHit::Target(MenuTarget::Bookmarks)
    );
}

#[test]
fn every_main_item_is_reachable() {
    let wanted = [
        MenuTarget::NewTab,
        MenuTarget::NewWindow,
        MenuTarget::History,
        MenuTarget::Bookmarks,
        MenuTarget::Downloads,
        MenuTarget::Passwords,
        MenuTarget::Settings,
        MenuTarget::Console,
        MenuTarget::Pages,
        MenuTarget::Exit,
    ];
    for target in wanted {
        let row = row_of(MenuKind::Main, MenuEntry::Item(target));
        assert_eq!(
            hit_menu(MenuKind::Main, centre(row), WIDTH, HEIGHT),
            MenuHit::Target(target)
        );
    }
}

#[test]
fn the_zoom_row_has_three_separate_buttons() {
    let row = row_of(MenuKind::Main, MenuEntry::Zoom);

    assert_eq!(
        hit_menu(MenuKind::Main, centre(zoom_out_rect(row)), WIDTH, HEIGHT),
        MenuHit::Target(MenuTarget::ZoomOut)
    );
    assert_eq!(
        hit_menu(MenuKind::Main, centre(zoom_value_rect(row)), WIDTH, HEIGHT),
        MenuHit::Target(MenuTarget::ZoomReset)
    );
    assert_eq!(
        hit_menu(MenuKind::Main, centre(zoom_in_rect(row)), WIDTH, HEIGHT),
        MenuHit::Target(MenuTarget::ZoomIn)
    );
    assert_eq!(
        hit_menu(
            MenuKind::Main,
            Point::new(row.origin.x + 6.0, row.origin.y + 6.0),
            WIDTH,
            HEIGHT
        ),
        MenuHit::Inert
    );
}

#[test]
fn a_separator_and_the_padding_do_nothing() {
    let rows = entry_rects(MenuKind::Main, WIDTH, HEIGHT);
    let separator = rows
        .iter()
        .find(|(entry, _)| *entry == MenuEntry::Separator)
        .map(|(_, rect)| *rect)
        .expect("separator");
    let popup = menu_rect(MenuKind::Main, WIDTH, HEIGHT);

    assert_eq!(
        hit_menu(MenuKind::Main, centre(separator), WIDTH, HEIGHT),
        MenuHit::Inert
    );
    assert_eq!(
        hit_menu(
            MenuKind::Main,
            Point::new(popup.origin.x + 1.0, popup.origin.y + 1.0),
            WIDTH,
            HEIGHT
        ),
        MenuHit::Inert
    );
}

#[test]
fn a_click_elsewhere_is_outside() {
    assert_eq!(
        hit_menu(MenuKind::Main, Point::new(20.0, 300.0), WIDTH, HEIGHT),
        MenuHit::Outside
    );
}

#[test]
fn the_start_menu_lists_the_three_shortcuts() {
    for target in [
        MenuTarget::Bookmarks,
        MenuTarget::Passwords,
        MenuTarget::Pages,
    ] {
        let row = row_of(MenuKind::Start, MenuEntry::Item(target));
        assert_eq!(
            hit_menu(MenuKind::Start, centre(row), WIDTH, HEIGHT),
            MenuHit::Target(target)
        );
    }
}

#[test]
fn the_menu_stays_inside_a_narrow_window() {
    let popup = menu_rect(MenuKind::Main, 200.0, 600.0);

    assert!(popup.origin.x >= 0.0);
}
