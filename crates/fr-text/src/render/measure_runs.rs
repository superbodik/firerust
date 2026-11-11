use crate::engine::text_engine::TextEngine;
use crate::render::content_widths::ContentWidths;
use crate::render::run_key::run_key;
use crate::render::shape_runs::shape_runs;
use crate::render::text_alignment::TextAlignment;
use crate::render::text_run::TextRun;

const UNBOUNDED_WIDTH: f32 = 1_000_000.0;

pub fn measure_runs(engine: &mut TextEngine, runs: &[TextRun<'_>]) -> ContentWidths {
    let key = run_key(runs, UNBOUNDED_WIDTH, TextAlignment::Left);
    if let Some(known) = engine.cached_widths(key) {
        return known;
    }
    let shaped = shape_runs(engine, runs, UNBOUNDED_WIDTH, TextAlignment::Left);
    let widest = shaped.width();
    let narrowest = shaped.longest_word_width();
    let widths = ContentWidths {
        min: narrowest.min(widest).ceil(),
        max: widest.ceil(),
    };
    engine.remember_widths(key, widths);
    widths
}
