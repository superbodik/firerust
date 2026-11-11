use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::value::float_side::FloatSide;
use fr_dom::node::node_id::NodeId;

use crate::block::block_spec::BlockSpec;
use crate::block::layout_block_box::layout_block_box;
use crate::context::layout_context::LayoutContext;
use crate::flow::float_band::FloatBand;
use crate::flow::flow_state::FlowState;
use crate::flow::narrowed_area::narrowed_area;
use crate::flow::nearest_float_bottom::nearest_float_bottom;
use crate::flow::shrink_width::shrink_width;
use crate::metrics::resolve_metrics::resolve_metrics;

pub fn place_float(
    context: &mut LayoutContext<'_>,
    state: &mut FlowState,
    id: NodeId,
    style: &ComputedStyle,
) {
    let available = state.area.width;
    let metrics = resolve_metrics(style, available);
    let mut top = state.cursor
        + if state.has_placed || !state.absorbs_top {
            state.pending_margin
        } else {
            0.0
        };
    let forced = shrink_width(context, id, style, available);
    let probe = layout_block_box(
        context,
        id,
        BlockSpec {
            x: 0.0,
            y: top + metrics.margin.top,
            available_width: available,
            forced_width: forced,
            forced_height: None,
        },
    );
    let border_width = probe.layout_box.border_box.size.width;
    let outer_width = border_width + metrics.margin.left + metrics.margin.right;
    let area = loop {
        let area = narrowed_area(state, top);
        if outer_width <= area.width + 0.5 {
            break area;
        }
        match nearest_float_bottom(state, top) {
            Some(next) => top = next,
            None => break area,
        }
    };
    let x = match style.float_side {
        FloatSide::Right => (area.x + area.width - outer_width).max(area.x),
        FloatSide::Left | FloatSide::None => area.x,
    };
    let result = layout_block_box(
        context,
        id,
        BlockSpec {
            x,
            y: top + metrics.margin.top,
            available_width: available,
            forced_width: Some(border_width),
            forced_height: None,
        },
    );
    state.floats.push(FloatBand {
        side: style.float_side,
        width: outer_width,
        bottom: result.layout_box.bottom() + result.margin_bottom,
    });
    state.boxes.push(result.layout_box);
}
