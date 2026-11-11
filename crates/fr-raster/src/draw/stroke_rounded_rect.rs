use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::rect::Rect;
use tiny_skia::{Stroke, Transform};

use crate::draw::rounded_rect_path::rounded_rect_path;
use crate::draw::smooth_paint::smooth_paint;
use crate::surface::canvas::Canvas;

pub fn stroke_rounded_rect(canvas: &mut Canvas, rect: Rect, radius: f32, width: f32, color: Rgba) {
    let half = width / 2.0;
    let inner = Rect::from_xywh(
        rect.origin.x + half,
        rect.origin.y + half,
        (rect.size.width - width).max(0.0),
        (rect.size.height - width).max(0.0),
    );
    let Some(path) = rounded_rect_path(inner, (radius - half).max(0.0)) else {
        return;
    };
    let stroke = Stroke {
        width,
        ..Stroke::default()
    };
    canvas.pixmap_mut().stroke_path(
        &path,
        &smooth_paint(color),
        &stroke,
        Transform::identity(),
        None,
    );
}
