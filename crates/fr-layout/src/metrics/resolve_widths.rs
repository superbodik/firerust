use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::value::box_sizing::BoxSizing;

use crate::metrics::box_metrics::BoxMetrics;
use crate::metrics::resolve_length::resolve_length;
use crate::metrics::resolved_widths::ResolvedWidths;

pub fn resolve_widths(
    style: &ComputedStyle,
    metrics: &BoxMetrics,
    available: f32,
    forced_border_width: Option<f32>,
) -> ResolvedWidths {
    let extras = metrics.horizontal_extras();
    if let Some(border_width) = forced_border_width {
        return ResolvedWidths {
            content: (border_width - extras).max(0.0),
            margin_left: metrics.margin.left,
            margin_right: metrics.margin.right,
        };
    }

    let inner = |value: f32| match style.box_sizing {
        BoxSizing::BorderBox => (value - extras).max(0.0),
        BoxSizing::ContentBox => value,
    };
    let specified = resolve_length(style.size.width, available).map(inner);
    let minimum = resolve_length(style.size.min_width, available).map(inner);
    let maximum = resolve_length(style.size.max_width, available).map(inner);

    let fill = available - metrics.margin.left - metrics.margin.right - extras;
    let mut content = specified.unwrap_or(fill);
    if let Some(limit) = maximum {
        content = content.min(limit);
    }
    if let Some(limit) = minimum {
        content = content.max(limit);
    }
    content = content.max(0.0);

    let mut margin_left = metrics.margin.left;
    let mut margin_right = metrics.margin.right;
    let remaining = available - content - extras - margin_left - margin_right;
    if remaining > 0.0 {
        match (metrics.is_margin_left_auto, metrics.is_margin_right_auto) {
            (true, true) => {
                margin_left += remaining / 2.0;
                margin_right += remaining / 2.0;
            }
            (true, false) => margin_left += remaining,
            (false, true) => margin_right += remaining,
            (false, false) => {}
        }
    }
    ResolvedWidths {
        content,
        margin_left,
        margin_right,
    }
}
