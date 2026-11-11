use fr_geom::geometry::rect::Rect;

use crate::menu::menu_metrics::{START_BUTTON_GAP, START_BUTTON_HEIGHT};

pub const START_BUTTON_WIDTH: f32 = 150.0;

pub fn start_button_rect(viewport_height: f32) -> Rect {
    Rect::from_xywh(
        START_BUTTON_GAP,
        (viewport_height - START_BUTTON_GAP - START_BUTTON_HEIGHT).max(0.0),
        START_BUTTON_WIDTH,
        START_BUTTON_HEIGHT,
    )
}
