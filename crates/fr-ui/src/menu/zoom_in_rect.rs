use fr_geom::geometry::rect::Rect;

pub const ZOOM_BUTTON: f32 = 30.0;
pub const ZOOM_INSET: f32 = 8.0;

pub fn zoom_in_rect(row: Rect) -> Rect {
    Rect::from_xywh(
        row.right() - ZOOM_INSET - ZOOM_BUTTON,
        row.origin.y + (row.size.height - ZOOM_BUTTON) / 2.0,
        ZOOM_BUTTON,
        ZOOM_BUTTON,
    )
}
