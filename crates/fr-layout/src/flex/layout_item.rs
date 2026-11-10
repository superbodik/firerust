use fr_geom::geometry::rect::Rect;

use crate::block::block_spec::BlockSpec;
use crate::block::layout_block_box::layout_block_box;
use crate::boxes::box_kind::BoxKind;
use crate::boxes::layout_box::LayoutBox;
use crate::context::layout_context::LayoutContext;
use crate::flex::flex_item::FlexItem;
use crate::flow::layout_flow_nodes::layout_flow_nodes;
use crate::metrics::content_area::ContentArea;
use crate::metrics::resolve_metrics::resolve_metrics;

pub fn layout_item(
    context: &mut LayoutContext<'_>,
    item: &FlexItem,
    border_box: Rect,
    container_width: f32,
) -> LayoutBox {
    match item {
        FlexItem::Element { node, style } => {
            let metrics = resolve_metrics(style, container_width);
            layout_block_box(
                context,
                *node,
                BlockSpec {
                    x: border_box.origin.x - metrics.margin.left,
                    y: border_box.origin.y,
                    available_width: container_width,
                    forced_width: Some(border_box.size.width),
                    forced_height: Some(border_box.size.height),
                },
            )
            .layout_box
        }
        FlexItem::Anonymous { nodes, style } => {
            let flow = layout_flow_nodes(
                context,
                nodes,
                style,
                ContentArea {
                    x: border_box.origin.x,
                    y: border_box.origin.y,
                    width: border_box.size.width,
                },
                false,
                false,
            );
            let height = border_box.size.height.max(flow.content_height);
            let rect = Rect::from_xywh(
                border_box.origin.x,
                border_box.origin.y,
                border_box.size.width,
                height,
            );
            let mut layout_box = LayoutBox::plain(None, BoxKind::Block, rect);
            layout_box.children = flow.boxes;
            layout_box
        }
    }
}
