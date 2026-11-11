use fr_geom::geometry::corner_radii::CornerRadii;
use fr_geom::geometry::rect::Rect;
use tiny_skia::{Path, PathBuilder};

use crate::draw::push_rounded_rect::push_rounded_rect;

pub fn rounded_rect_path(rect: Rect, radius: f32) -> Option<Path> {
    rounded_corners_path(rect, CornerRadii::uniform(radius.max(0.0)))
}

pub fn rounded_corners_path(rect: Rect, radii: CornerRadii) -> Option<Path> {
    let mut builder = PathBuilder::new();
    push_rounded_rect(&mut builder, rect, radii);
    builder.finish()
}
