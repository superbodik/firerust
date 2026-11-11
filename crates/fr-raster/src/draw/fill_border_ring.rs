use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::corner_radii::CornerRadii;
use fr_geom::geometry::rect::Rect;
use tiny_skia::{FillRule, PathBuilder, Transform};

use crate::draw::push_rounded_rect::push_rounded_rect;
use crate::draw::smooth_paint::smooth_paint;
use crate::surface::canvas::Canvas;

pub fn fill_border_ring(
    canvas: &mut Canvas,
    rect: Rect,
    radii: CornerRadii,
    widths: [f32; 4],
    color: Rgba,
) {
    let [top, right, bottom, left] = widths;
    let radii = radii.fitted(rect.size.width, rect.size.height);
    let inner = Rect::from_xywh(
        rect.origin.x + left,
        rect.origin.y + top,
        (rect.size.width - left - right).max(0.0),
        (rect.size.height - top - bottom).max(0.0),
    );
    let mut builder = PathBuilder::new();
    push_rounded_rect(&mut builder, rect, radii);
    if inner.size.width > 0.0 && inner.size.height > 0.0 {
        push_rounded_rect(&mut builder, inner, radii.inset(top, right, bottom, left));
    }
    let Some(path) = builder.finish() else {
        return;
    };
    canvas.pixmap_mut().fill_path(
        &path,
        &smooth_paint(color),
        FillRule::EvenOdd,
        Transform::identity(),
        None,
    );
}
