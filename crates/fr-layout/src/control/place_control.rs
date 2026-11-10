use fr_css::cascade::computed_style::ComputedStyle;
use fr_dom::node::node_id::NodeId;

use crate::context::layout_context::LayoutContext;
use crate::control::build_control_box::build_control_box;
use crate::control::control_kind::ControlKind;
use crate::flow::flow_state::FlowState;
use crate::inline::flush_inline::flush_inline;
use crate::metrics::resolve_metrics::resolve_metrics;

const CONTROL_SPACING: f32 = 4.0;

pub fn place_control(
    context: &mut LayoutContext<'_>,
    state: &mut FlowState,
    node: NodeId,
    kind: ControlKind,
    style: &ComputedStyle,
    container: &ComputedStyle,
) {
    flush_inline(context, state, container);
    state.cursor += state.pending_margin;
    state.pending_margin = 0.0;
    let metrics = resolve_metrics(style, state.area.width);
    let boxed = build_control_box(
        context,
        node,
        kind,
        style,
        state.area.x + metrics.margin.left,
        state.cursor + metrics.margin.top,
        state.area.width,
    );
    state.cursor = boxed.bottom() + metrics.margin.bottom.max(CONTROL_SPACING);
    state.has_placed = true;
    state.boxes.push(boxed);
}
