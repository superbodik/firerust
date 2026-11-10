use fr_geom::color::rgba::Rgba;
use tiny_skia::Paint;

pub fn paint_for(color: Rgba) -> Paint<'static> {
    let mut paint = Paint::default();
    paint.set_color_rgba8(color.r, color.g, color.b, color.a);
    paint.anti_alias = false;
    paint
}
