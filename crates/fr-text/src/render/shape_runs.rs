use cosmic_text::{Attrs, Buffer, Family, Shaping};

use crate::engine::text_engine::TextEngine;
use crate::render::line_metrics::{DEFAULT_FONT_SIZE, metrics_for};
use crate::render::shaped_text::ShapedText;
use crate::render::span_attrs::span_attrs;
use crate::render::text_run::TextRun;

pub fn shape_runs(engine: &mut TextEngine, runs: &[TextRun<'_>], max_width: f32) -> ShapedText {
    let (font_system, _) = engine.parts_mut();
    let mut buffer = Buffer::new(font_system, metrics_for(DEFAULT_FONT_SIZE));
    {
        let mut shaping = buffer.borrow_with(font_system);
        shaping.set_size(Some(max_width), None);
        let default_attrs = Attrs::new().family(Family::SansSerif);
        shaping.set_rich_text(
            runs.iter().map(|run| (run.text, span_attrs(run))),
            &default_attrs,
            Shaping::Advanced,
            None,
        );
        shaping.shape_until_scroll(false);
    }
    ShapedText::new(buffer)
}
