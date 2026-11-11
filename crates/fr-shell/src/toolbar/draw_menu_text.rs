use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::point::Point;
use fr_geom::geometry::rect::Rect;
use fr_raster::surface::canvas::Canvas;
use fr_text::engine::text_engine::TextEngine;
use fr_text::render::shape_runs::shape_runs;
use fr_text::render::text_alignment::TextAlignment;

use crate::toolbar::toolbar_run::toolbar_run;

pub fn draw_menu_text(
    canvas: &mut Canvas,
    engine: &mut TextEngine,
    text: &str,
    font_size: f32,
    color: Rgba,
    left: f32,
    row: Rect,
) {
    if text.is_empty() {
        return;
    }
    let run = toolbar_run(text, font_size, color);
    let mut shaped = shape_runs(engine, &[run], row.size.width, TextAlignment::Left);
    let top = row.origin.y + (row.size.height - shaped.height()) / 2.0;
    shaped.draw(engine, canvas, Point::new(left, top));
}
