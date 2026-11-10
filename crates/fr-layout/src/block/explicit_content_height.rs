use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::value::box_sizing::BoxSizing;
use fr_css::value::length::Length;

use crate::metrics::box_metrics::BoxMetrics;

pub fn explicit_content_height(
    style: &ComputedStyle,
    metrics: &BoxMetrics,
    forced_border_height: Option<f32>,
) -> Option<f32> {
    let extras = metrics.vertical_extras();
    if let Some(forced) = forced_border_height {
        return Some((forced - extras).max(0.0));
    }
    match style.size.height {
        Length::Px(value) => Some(match style.box_sizing {
            BoxSizing::BorderBox => (value - extras).max(0.0),
            BoxSizing::ContentBox => value.max(0.0),
        }),
        _ => None,
    }
}
