use fr_dom::node::node_id::NodeId;

use crate::block::block_spec::BlockSpec;
use crate::block::layout_block_box::layout_block_box;
use crate::context::layout_context::LayoutContext;
use crate::flow::available_area::available_area;
use crate::flow::effective_margin_top::effective_margin_top;
use crate::flow::flow_state::FlowState;
use crate::metrics::collapse_margins::collapse_margins;

pub fn place_block(context: &mut LayoutContext<'_>, state: &mut FlowState, node: NodeId) {
    let top = effective_margin_top(context, node, state.area.width);
    let gap = if !state.has_placed && state.absorbs_top {
        0.0
    } else {
        collapse_margins(state.pending_margin, top)
    };
    state.cursor += gap;
    state.pending_margin = 0.0;
    let area = available_area(state);
    let result = layout_block_box(
        context,
        node,
        BlockSpec {
            x: area.x,
            y: state.cursor,
            available_width: area.width,
            forced_width: None,
            forced_height: None,
        },
    );
    state.cursor = result.layout_box.bottom();
    state.pending_margin = result.margin_bottom;
    state.has_placed = true;
    state.boxes.push(result.layout_box);
}
