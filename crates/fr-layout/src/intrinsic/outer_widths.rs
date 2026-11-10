use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::value::box_sizing::BoxSizing;
use fr_css::value::length::Length;
use fr_dom::node::node_id::NodeId;
use fr_text::render::content_widths::ContentWidths;

use crate::context::layout_context::LayoutContext;
use crate::intrinsic::content_widths_of::content_widths_of;

pub fn outer_widths(
    context: &mut LayoutContext<'_>,
    id: NodeId,
    style: &ComputedStyle,
) -> ContentWidths {
    let inset = style.border_width.left
        + style.border_width.right
        + style.padding.left.px_or_zero()
        + style.padding.right.px_or_zero();
    let margin = style.margin.left.px_or_zero() + style.margin.right.px_or_zero();
    let content = match style.size.width {
        Length::Px(value) => {
            let inner = match style.box_sizing {
                BoxSizing::BorderBox => (value - inset).max(0.0),
                BoxSizing::ContentBox => value,
            };
            ContentWidths {
                min: inner,
                max: inner,
            }
        }
        _ => content_widths_of(context, id),
    };
    ContentWidths {
        min: content.min + inset + margin,
        max: content.max + inset + margin,
    }
}
