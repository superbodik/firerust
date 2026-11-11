use fr_geom::geometry::rect::Rect;
use tiny_skia::{Path, PathBuilder};

const KAPPA: f32 = 0.552_284_8;

pub fn rounded_rect_path(rect: Rect, radius: f32) -> Option<Path> {
    let left = rect.origin.x;
    let top = rect.origin.y;
    let right = rect.right();
    let bottom = rect.bottom();
    let r = radius
        .min(rect.size.width / 2.0)
        .min(rect.size.height / 2.0)
        .max(0.0);
    let k = r * KAPPA;
    let mut builder = PathBuilder::new();
    builder.move_to(left + r, top);
    builder.line_to(right - r, top);
    builder.cubic_to(right - r + k, top, right, top + r - k, right, top + r);
    builder.line_to(right, bottom - r);
    builder.cubic_to(
        right,
        bottom - r + k,
        right - r + k,
        bottom,
        right - r,
        bottom,
    );
    builder.line_to(left + r, bottom);
    builder.cubic_to(left + r - k, bottom, left, bottom - r + k, left, bottom - r);
    builder.line_to(left, top + r);
    builder.cubic_to(left, top + r - k, left + r - k, top, left + r, top);
    builder.close();
    builder.finish()
}
