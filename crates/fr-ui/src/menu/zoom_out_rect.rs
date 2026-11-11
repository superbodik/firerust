use fr_geom::geometry::rect::Rect;

use crate::menu::zoom_in_rect::ZOOM_BUTTON;
use crate::menu::zoom_value_rect::zoom_value_rect;

pub fn zoom_out_rect(row: Rect) -> Rect {
    let value = zoom_value_rect(row);
    Rect::from_xywh(
        value.origin.x - 4.0 - ZOOM_BUTTON,
        value.origin.y,
        ZOOM_BUTTON,
        ZOOM_BUTTON,
    )
}
