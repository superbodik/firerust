use taffy::{AvailableSpace, Size};

use crate::context::layout_context::LayoutContext;
use crate::flex::content_height_at::content_height_at;
use crate::flex::flex_item::FlexItem;
use crate::intrinsic::content_widths_of::content_widths_of;
use crate::intrinsic::content_widths_of_nodes::content_widths_of_nodes;

pub fn measure_item(
    context: &mut LayoutContext<'_>,
    item: &FlexItem,
    container_width: f32,
    known: Size<Option<f32>>,
    available: Size<AvailableSpace>,
) -> Size<f32> {
    let widths = match item {
        FlexItem::Element { node, .. } => content_widths_of(context, *node),
        FlexItem::Anonymous { nodes, style } => content_widths_of_nodes(context, nodes, style),
    };
    let width = known
        .width
        .unwrap_or_else(|| match available.width {
            AvailableSpace::Definite(space) => widths.max.min(space),
            AvailableSpace::MinContent => widths.min,
            AvailableSpace::MaxContent => widths.max,
        })
        .max(0.0);
    let height = known
        .height
        .unwrap_or_else(|| content_height_at(context, item, width, container_width));
    Size { width, height }
}
