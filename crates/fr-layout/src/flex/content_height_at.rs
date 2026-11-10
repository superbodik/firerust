use fr_dom::node::node_id::NodeId;

use crate::block::block_spec::BlockSpec;
use crate::block::layout_block_box::layout_block_box;
use crate::context::layout_context::LayoutContext;
use crate::flex::flex_item::FlexItem;
use crate::flow::layout_flow_nodes::layout_flow_nodes;
use crate::metrics::content_area::ContentArea;
use crate::metrics::resolve_metrics::resolve_metrics;

pub fn content_height_at(
    context: &mut LayoutContext<'_>,
    item: &FlexItem,
    content_width: f32,
    container_width: f32,
) -> f32 {
    match item {
        FlexItem::Element { node, style } => {
            let metrics = resolve_metrics(style, container_width);
            let border_width = content_width + metrics.horizontal_extras();
            let result = layout_block_box(
                context,
                NodeId::clone(node),
                BlockSpec {
                    x: 0.0,
                    y: 0.0,
                    available_width: container_width,
                    forced_width: Some(border_width),
                    forced_height: None,
                },
            );
            (result.layout_box.border_box.size.height - metrics.vertical_extras()).max(0.0)
        }
        FlexItem::Anonymous { nodes, style } => {
            layout_flow_nodes(
                context,
                nodes,
                style,
                ContentArea {
                    x: 0.0,
                    y: 0.0,
                    width: content_width,
                },
                false,
                false,
            )
            .content_height
        }
    }
}
