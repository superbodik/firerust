use cosmic_text::{Align, Attrs, Buffer, Family, Shaping};

use crate::engine::text_engine::TextEngine;
use crate::render::line_metrics::{DEFAULT_FONT_SIZE, metrics_for};
use crate::render::shaped_text::ShapedText;
use crate::render::span_attrs::span_attrs;
use crate::render::text_alignment::TextAlignment;
use crate::render::text_run::TextRun;

pub fn shape_runs(
    engine: &mut TextEngine,
    runs: &[TextRun<'_>],
    max_width: f32,
    alignment: TextAlignment,
) -> ShapedText {
    let (font_system, _) = engine.parts_mut();
    let base_height = DEFAULT_FONT_SIZE * 1.4;
    let mut buffer = Buffer::new(font_system, metrics_for(DEFAULT_FONT_SIZE, base_height));
    {
        let mut shaping = buffer.borrow_with(font_system);
        shaping.set_size(Some(max_width), None);
        let default_attrs = Attrs::new().family(Family::SansSerif);
        let align = match alignment {
            TextAlignment::Left => Align::Left,
            TextAlignment::Center => Align::Center,
            TextAlignment::Right => Align::Right,
        };
        shaping.set_rich_text(
            runs.iter().map(|run| (run.text, span_attrs(run))),
            &default_attrs,
            Shaping::Advanced,
            Some(align),
        );
        shaping.shape_until_scroll(false);
    }
    ShapedText::new(buffer)
}
