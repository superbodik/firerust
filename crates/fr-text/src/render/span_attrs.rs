use cosmic_text::{Attrs, Color as GlyphColor, Family, Style, UnderlineStyle, Weight};

use crate::render::line_metrics::metrics_for;
use crate::render::text_family::TextFamily;
use crate::render::text_run::TextRun;

pub fn span_attrs(run: &TextRun<'_>) -> Attrs<'static> {
    let family = match run.family {
        TextFamily::SansSerif => Family::SansSerif,
        TextFamily::Serif => Family::Serif,
        TextFamily::Monospace => Family::Monospace,
    };
    let style = if run.is_italic {
        Style::Italic
    } else {
        Style::Normal
    };
    let mut attrs = Attrs::new()
        .family(family)
        .style(style)
        .color(GlyphColor::rgba(
            run.color.r,
            run.color.g,
            run.color.b,
            run.color.a,
        ))
        .weight(Weight(run.font_weight))
        .metrics(metrics_for(run.font_size, run.line_height));
    if run.is_underlined {
        attrs = attrs.underline(UnderlineStyle::Single);
    }
    if run.is_struck {
        attrs = attrs.strikethrough();
    }
    attrs
}
