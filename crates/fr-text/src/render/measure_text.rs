use crate::engine::text_engine::TextEngine;
use crate::render::run_key::run_key;
use crate::render::shape_runs::shape_runs;
use crate::render::text_alignment::TextAlignment;
use crate::render::text_metrics::TextMetrics;
use crate::render::text_run::TextRun;

pub fn measure_text(
    engine: &mut TextEngine,
    runs: &[TextRun<'_>],
    max_width: f32,
    alignment: TextAlignment,
) -> TextMetrics {
    let key = run_key(runs, max_width, alignment);
    if let Some(known) = engine.cached_metrics(key) {
        return known;
    }
    let shaped = shape_runs(engine, runs, max_width, alignment);
    let metrics = TextMetrics {
        height: shaped.height(),
        width: shaped.width(),
    };
    engine.remember_metrics(key, metrics);
    metrics
}
