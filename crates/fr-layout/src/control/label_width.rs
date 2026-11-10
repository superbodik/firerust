use fr_css::cascade::computed_style::ComputedStyle;
use fr_text::engine::text_engine::TextEngine;
use fr_text::render::measure_runs::measure_runs;

use crate::boxes::layout_run::LayoutRun;

pub fn label_width(engine: &mut TextEngine, text: &str, style: &ComputedStyle) -> f32 {
    if text.is_empty() {
        return 0.0;
    }
    let run = LayoutRun::from_style(text.to_owned(), style);
    measure_runs(engine, &[run.as_text_run()]).max
}
