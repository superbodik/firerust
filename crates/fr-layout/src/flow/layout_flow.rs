use fr_css::cascade::computed_style::ComputedStyle;
use fr_dom::node::node_id::NodeId;

use crate::context::layout_context::LayoutContext;
use crate::flow::flow_result::FlowResult;
use crate::flow::layout_flow_nodes::layout_flow_nodes;
use crate::metrics::content_area::ContentArea;

pub fn layout_flow(
    context: &mut LayoutContext<'_>,
    node: NodeId,
    style: &ComputedStyle,
    area: ContentArea,
    absorbs_top: bool,
    absorbs_bottom: bool,
) -> FlowResult {
    let children: Vec<NodeId> = context.document().children(node).collect();
    layout_flow_nodes(context, &children, style, area, absorbs_top, absorbs_bottom)
}
