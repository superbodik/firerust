use cosmic_text::{Attrs, Color as GlyphColor, Family, Weight};

use crate::render::line_metrics::metrics_for;
use crate::render::text_run::TextRun;

pub fn span_attrs(run: &TextRun<'_>) -> Attrs<'static> {
    Attrs::new()
        .family(Family::SansSerif)
        .color(GlyphColor::rgba(
            run.color.r,
            run.color.g,
            run.color.b,
            run.color.a,
        ))
        .weight(Weight(run.font_weight))
        .metrics(metrics_for(run.font_size))
}
