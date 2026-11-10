use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::value::flex_direction::FlexDirection;
use fr_dom::node::node_id::NodeId;
use fr_text::render::content_widths::ContentWidths;

use crate::context::layout_context::LayoutContext;
use crate::flex::collect_flex_items::collect_flex_items;
use crate::flex::flex_item::FlexItem;
use crate::intrinsic::content_widths_of_nodes::content_widths_of_nodes;
use crate::intrinsic::outer_widths::outer_widths;

pub fn flex_widths(
    context: &mut LayoutContext<'_>,
    node: NodeId,
    style: &ComputedStyle,
) -> ContentWidths {
    let items = collect_flex_items(context, node, style);
    let outer: Vec<ContentWidths> = items
        .iter()
        .map(|item| match item {
            FlexItem::Element { node, style } => outer_widths(context, *node, style),
            FlexItem::Anonymous { nodes, style } => content_widths_of_nodes(context, nodes, style),
        })
        .collect();
    if outer.is_empty() {
        return ContentWidths { min: 0.0, max: 0.0 };
    }

    let is_row = matches!(
        style.flex.direction,
        FlexDirection::Row | FlexDirection::RowReverse
    );
    let largest_min = outer
        .iter()
        .fold(0.0_f32, |acc, widths| acc.max(widths.min));
    let largest_max = outer
        .iter()
        .fold(0.0_f32, |acc, widths| acc.max(widths.max));
    if !is_row {
        return ContentWidths {
            min: largest_min,
            max: largest_max,
        };
    }
    let gaps = style.flex.column_gap * (outer.len() as f32 - 1.0);
    let total_max: f32 = outer.iter().map(|widths| widths.max).sum::<f32>() + gaps;
    let min = if style.flex.is_wrapping {
        largest_min
    } else {
        outer.iter().map(|widths| widths.min).sum::<f32>() + gaps
    };
    ContentWidths {
        min,
        max: total_max,
    }
}
