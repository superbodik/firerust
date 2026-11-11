use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::rect::Rect;
use tiny_skia::{FillRule, Transform};

use crate::draw::rounded_rect_path::rounded_rect_path;
use crate::draw::smooth_paint::smooth_paint;
use crate::surface::canvas::Canvas;

pub fn fill_rounded_rect(canvas: &mut Canvas, rect: Rect, radius: f32, color: Rgba) {
    let Some(path) = rounded_rect_path(rect, radius) else {
        return;
    };
    canvas.pixmap_mut().fill_path(
        &path,
        &smooth_paint(color),
        FillRule::Winding,
        Transform::identity(),
        None,
    );
}
