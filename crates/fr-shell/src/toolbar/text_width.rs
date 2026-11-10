use fr_geom::color::rgba::Rgba;
use fr_text::engine::text_engine::TextEngine;
use fr_text::render::shape_runs::shape_runs;
use fr_text::render::text_alignment::TextAlignment;

use crate::toolbar::toolbar_run::toolbar_run;

const UNBOUNDED_WIDTH: f32 = 100_000.0;

pub fn text_width(engine: &mut TextEngine, text: &str, font_size: f32) -> f32 {
    if text.is_empty() {
        return 0.0;
    }
    let run = toolbar_run(text, font_size, Rgba::BLACK);
    shape_runs(engine, &[run], UNBOUNDED_WIDTH, TextAlignment::Left).width()
}
