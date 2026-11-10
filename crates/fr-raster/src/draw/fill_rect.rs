use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::rect::Rect;
use tiny_skia::{Rect as SkiaRect, Transform};

use crate::draw::paint_for::paint_for;
use crate::surface::canvas::Canvas;

pub fn fill_rect(canvas: &mut Canvas, rect: Rect, color: Rgba) {
    let Some(skia_rect) = SkiaRect::from_xywh(
        rect.origin.x,
        rect.origin.y,
        rect.size.width,
        rect.size.height,
    ) else {
        return;
    };
    canvas
        .pixmap_mut()
        .fill_rect(skia_rect, &paint_for(color), Transform::identity(), None);
}
