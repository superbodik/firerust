use fr_geom::color::rgba::Rgba;
use tiny_skia::Paint;

pub fn smooth_paint(color: Rgba) -> Paint<'static> {
    let mut paint = Paint::default();
    paint.set_color_rgba8(color.r, color.g, color.b, color.a);
    paint.anti_alias = true;
    paint
}
