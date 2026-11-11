use fr_geom::geometry::rect::Rect;

use crate::menu::zoom_in_rect::{ZOOM_BUTTON, zoom_in_rect};

pub const VALUE_WIDTH: f32 = 58.0;

pub fn zoom_value_rect(row: Rect) -> Rect {
    let plus = zoom_in_rect(row);
    Rect::from_xywh(
        plus.origin.x - 4.0 - VALUE_WIDTH,
        plus.origin.y,
        VALUE_WIDTH,
        ZOOM_BUTTON,
    )
}
