use fr_window::app::resize_edge::{cursor_for_edge, resize_edge};
use winit::window::{CursorIcon, ResizeDirection};

const WIDTH: f32 = 1000.0;
const HEIGHT: f32 = 700.0;

fn edge(x: f32, y: f32) -> Option<ResizeDirection> {
    resize_edge(x, y, WIDTH, HEIGHT)
}

#[test]
fn the_middle_of_the_window_is_not_an_edge() {
    assert_eq!(edge(500.0, 350.0), None);
    assert_eq!(edge(20.0, 20.0), None);
}

#[test]
fn the_four_sides_are_found() {
    assert_eq!(edge(2.0, 350.0), Some(ResizeDirection::West));
    assert_eq!(edge(998.0, 350.0), Some(ResizeDirection::East));
    assert_eq!(edge(500.0, 2.0), Some(ResizeDirection::North));
    assert_eq!(edge(500.0, 698.0), Some(ResizeDirection::South));
}

#[test]
fn the_corners_win_over_the_sides() {
    assert_eq!(edge(2.0, 2.0), Some(ResizeDirection::NorthWest));
    assert_eq!(edge(998.0, 2.0), Some(ResizeDirection::NorthEast));
    assert_eq!(edge(2.0, 698.0), Some(ResizeDirection::SouthWest));
    assert_eq!(edge(998.0, 698.0), Some(ResizeDirection::SouthEast));
}

#[test]
fn a_corner_reaches_a_bit_further_along_the_edge() {
    assert_eq!(edge(10.0, 2.0), Some(ResizeDirection::NorthWest));
    assert_eq!(edge(2.0, 10.0), Some(ResizeDirection::NorthWest));
    assert_eq!(edge(30.0, 2.0), Some(ResizeDirection::North));
}

#[test]
fn points_outside_the_window_are_not_edges() {
    assert_eq!(edge(-1.0, 100.0), None);
    assert_eq!(edge(100.0, 701.0), None);
}

#[test]
fn every_direction_has_a_matching_cursor() {
    assert_eq!(
        cursor_for_edge(ResizeDirection::North),
        CursorIcon::NsResize
    );
    assert_eq!(cursor_for_edge(ResizeDirection::West), CursorIcon::EwResize);
    assert_eq!(
        cursor_for_edge(ResizeDirection::NorthEast),
        CursorIcon::NeswResize
    );
    assert_eq!(
        cursor_for_edge(ResizeDirection::SouthEast),
        CursorIcon::NwseResize
    );
}
