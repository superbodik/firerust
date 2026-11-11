use winit::window::{CursorIcon, ResizeDirection};

const BORDER: f32 = 6.0;
const CORNER: f32 = 14.0;

fn side(position: f32, extent: f32, reach: f32) -> Option<bool> {
    if position < reach {
        Some(false)
    } else if position > extent - reach {
        Some(true)
    } else {
        None
    }
}

pub fn resize_edge(x: f32, y: f32, width: f32, height: f32) -> Option<ResizeDirection> {
    if x < 0.0 || y < 0.0 || x > width || y > height {
        return None;
    }
    let mut horizontal = side(x, width, BORDER);
    let mut vertical = side(y, height, BORDER);
    if horizontal.is_none() && vertical.is_some() {
        horizontal = side(x, width, CORNER);
    }
    if vertical.is_none() && horizontal.is_some() {
        vertical = side(y, height, CORNER);
    }
    match (horizontal, vertical) {
        (Some(false), Some(false)) => Some(ResizeDirection::NorthWest),
        (Some(true), Some(false)) => Some(ResizeDirection::NorthEast),
        (Some(false), Some(true)) => Some(ResizeDirection::SouthWest),
        (Some(true), Some(true)) => Some(ResizeDirection::SouthEast),
        (Some(false), None) => Some(ResizeDirection::West),
        (Some(true), None) => Some(ResizeDirection::East),
        (None, Some(false)) => Some(ResizeDirection::North),
        (None, Some(true)) => Some(ResizeDirection::South),
        (None, None) => None,
    }
}

pub fn cursor_for_edge(direction: ResizeDirection) -> CursorIcon {
    match direction {
        ResizeDirection::North | ResizeDirection::South => CursorIcon::NsResize,
        ResizeDirection::East | ResizeDirection::West => CursorIcon::EwResize,
        ResizeDirection::NorthEast | ResizeDirection::SouthWest => CursorIcon::NeswResize,
        ResizeDirection::NorthWest | ResizeDirection::SouthEast => CursorIcon::NwseResize,
    }
}
