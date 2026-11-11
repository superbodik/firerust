use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::value::length::Length;
use fr_dom::node::node_id::NodeId;

use crate::context::layout_context::LayoutContext;
use crate::image::is_replaced::is_replaced;
use crate::intrinsic::outer_widths::outer_widths;

pub fn shrink_width(
    context: &mut LayoutContext<'_>,
    id: NodeId,
    style: &ComputedStyle,
    available: f32,
) -> Option<f32> {
    if !matches!(style.size.width, Length::Auto) || is_replaced(context.document(), id) {
        return None;
    }
    let widths = outer_widths(context, id, style);
    let margins = style.margin.left.px_or_zero() + style.margin.right.px_or_zero();
    let widest = (widths.max - margins).min(available - margins);
    let narrowest = widths.min - margins;
    Some(widest.max(narrowest).max(0.0))
}
