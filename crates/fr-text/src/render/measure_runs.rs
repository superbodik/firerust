use crate::engine::text_engine::TextEngine;
use crate::render::content_widths::ContentWidths;
use crate::render::shape_runs::shape_runs;
use crate::render::text_alignment::TextAlignment;
use crate::render::text_run::TextRun;

const UNBOUNDED_WIDTH: f32 = 1_000_000.0;
const NARROWEST_WIDTH: f32 = 1.0;

pub fn measure_runs(engine: &mut TextEngine, runs: &[TextRun<'_>]) -> ContentWidths {
    let widest = shape_runs(engine, runs, UNBOUNDED_WIDTH, TextAlignment::Left).width();
    let narrowest = shape_runs(engine, runs, NARROWEST_WIDTH, TextAlignment::Left).width();
    ContentWidths {
        min: narrowest.min(widest).ceil(),
        max: widest.ceil(),
    }
}
