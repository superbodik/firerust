use fr_geom::color::rgba::Rgba;
use tiny_skia::Color;

use crate::surface::canvas::Canvas;

pub fn clear(canvas: &mut Canvas, color: Rgba) {
    canvas
        .pixmap_mut()
        .fill(Color::from_rgba8(color.r, color.g, color.b, color.a));
}
