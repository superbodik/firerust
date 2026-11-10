use fr_css::cascade::computed_style::ComputedStyle;
use fr_dom::node::node_id::NodeId;

use crate::block::block_spec::BlockSpec;
use crate::context::layout_context::LayoutContext;
use crate::flow::flow_state::FlowState;
use crate::image::layout_replaced::layout_replaced;
use crate::inline::alignment_of::alignment_of;
use crate::inline::flush_inline::flush_inline;

pub fn place_inline_image(
    context: &mut LayoutContext<'_>,
    state: &mut FlowState,
    id: NodeId,
    style: &ComputedStyle,
    container: &ComputedStyle,
) {
    if context.images().get(id).is_none() {
        let alt = context
            .document()
            .node(id)
            .and_then(|node| node.kind().as_element())
            .and_then(|element| element.attribute("alt"))
            .unwrap_or_default();
        state.inline.push_text(alt, style);
        return;
    }
    flush_inline(context, state, container);
    state.cursor += state.pending_margin;
    state.pending_margin = 0.0;
    let result = layout_replaced(
        context,
        id,
        style,
        BlockSpec {
            x: state.area.x,
            y: state.cursor,
            available_width: state.area.width,
            forced_width: None,
            forced_height: None,
        },
        alignment_of(container.text_align),
    );
    state.cursor = result.layout_box.bottom() + result.margin_bottom;
    state.has_placed = true;
    state.boxes.push(result.layout_box);
}
