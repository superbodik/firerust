use fr_css::cascade::computed_style::ComputedStyle;
use fr_dom::node::node_id::NodeId;

use crate::context::layout_context::LayoutContext;
use crate::flow::flow_result::FlowResult;
use crate::flow::flow_state::FlowState;
use crate::flow::handle_node::handle_node;
use crate::inline::flush_inline::flush_inline;
use crate::metrics::content_area::ContentArea;

pub fn layout_flow(
    context: &mut LayoutContext<'_>,
    node: NodeId,
    style: &ComputedStyle,
    area: ContentArea,
    absorbs_top: bool,
    absorbs_bottom: bool,
) -> FlowResult {
    let document = context.document();
    let mut state = FlowState::new(area, absorbs_top);
    for child in document.children(node) {
        handle_node(context, &mut state, child, style);
    }
    flush_inline(context, &mut state, style);

    let trailing = state.pending_margin;
    let (content_bottom, margin_bottom_out) = if absorbs_bottom {
        (state.cursor, trailing)
    } else {
        (state.cursor + trailing, 0.0)
    };
    FlowResult {
        boxes: state.boxes,
        content_height: (content_bottom - area.y).max(0.0),
        margin_bottom_out,
    }
}
