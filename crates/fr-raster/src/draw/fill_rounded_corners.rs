use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::corner_radii::CornerRadii;
use fr_geom::geometry::rect::Rect;
use tiny_skia::{FillRule, Transform};

use crate::draw::rounded_rect_path::rounded_corners_path;
use crate::draw::smooth_paint::smooth_paint;
use crate::surface::canvas::Canvas;

pub fn fill_rounded_corners(canvas: &mut Canvas, rect: Rect, radii: CornerRadii, color: Rgba) {
    let Some(path) = rounded_corners_path(rect, radii) else {
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
