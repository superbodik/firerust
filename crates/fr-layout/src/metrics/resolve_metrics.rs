use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::value::length::Length;

use crate::metrics::box_metrics::BoxMetrics;
use crate::metrics::resolve_length::resolve_length;

pub fn resolve_metrics(style: &ComputedStyle, base_width: f32) -> BoxMetrics {
    let resolve = |length: Length| resolve_length(length, base_width).unwrap_or(0.0);
    BoxMetrics {
        margin: style.margin.map(resolve),
        is_margin_left_auto: style.margin.left == Length::Auto,
        is_margin_right_auto: style.margin.right == Length::Auto,
        padding: style.padding.map(|length| resolve(length).max(0.0)),
        border: style.border_width,
    }
}
