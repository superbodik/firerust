use fr_geom::geometry::point::Point;
use fr_raster::surface::canvas::Canvas;

use crate::engine::text_engine::TextEngine;
use crate::render::paint_key::paint_key;
use crate::render::shape_runs::shape_runs;
use crate::render::text_alignment::TextAlignment;
use crate::render::text_run::TextRun;

pub fn draw_text(
    engine: &mut TextEngine,
    canvas: &mut Canvas,
    runs: &[TextRun<'_>],
    width: f32,
    alignment: TextAlignment,
    origin: Point,
) {
    let key = paint_key(runs, width, alignment);
    let mut shaped = match engine.take_shaped(key) {
        Some(known) => known,
        None => shape_runs(engine, runs, width, alignment),
    };
    shaped.draw(engine, canvas, origin);
    engine.keep_shaped(key, shaped);
}
