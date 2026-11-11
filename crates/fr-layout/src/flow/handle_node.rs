use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::value::display::Display;
use fr_css::value::float_side::FloatSide;
use fr_dom::node::node_id::NodeId;
use fr_dom::query::element_name::element_name;

use crate::context::layout_context::LayoutContext;
use crate::control::control_kind_of::control_kind_of;
use crate::control::place_control::place_control;
use crate::flow::apply_clear::apply_clear;
use crate::flow::flow_state::FlowState;
use crate::flow::place_block::place_block;
use crate::flow::place_float::place_float;
use crate::image::place_inline_image::place_inline_image;
use crate::inline::flush_inline::flush_inline;

pub fn handle_node(
    context: &mut LayoutContext<'_>,
    state: &mut FlowState,
    id: NodeId,
    container: &ComputedStyle,
) {
    let document = context.document();
    let styles = context.styles();
    let Some(node) = document.node(id) else {
        return;
    };
    if let Some(text) = node.kind().as_text() {
        let style = node
            .parent()
            .and_then(|parent| styles.get(parent))
            .unwrap_or(container);
        state.inline.push_text(text, style, node.parent());
        return;
    }
    let Some(style) = styles.get(id) else {
        return;
    };
    match style.display {
        Display::None => {}
        _ if style.float_side != FloatSide::None => {
            flush_inline(context, state, container);
            apply_clear(state, style.clear_side);
            place_float(context, state, id, style);
        }
        _ if is_form_field(context, id) => {
            place_form_field(context, state, id, style, container);
        }
        Display::Inline => handle_inline(context, state, id, style, container),
        _ => {
            flush_inline(context, state, container);
            apply_clear(state, style.clear_side);
            place_block(context, state, id);
        }
    }
}

fn is_form_field(context: &LayoutContext<'_>, id: NodeId) -> bool {
    let document = context.document();
    matches!(
        element_name(document, id),
        Some("input" | "textarea" | "select")
    )
}

fn place_form_field(
    context: &mut LayoutContext<'_>,
    state: &mut FlowState,
    id: NodeId,
    style: &ComputedStyle,
    container: &ComputedStyle,
) {
    if let Some(kind) = control_kind_of(context.document(), id) {
        place_control(context, state, id, kind, style, container);
    }
}

fn handle_inline(
    context: &mut LayoutContext<'_>,
    state: &mut FlowState,
    id: NodeId,
    style: &ComputedStyle,
    container: &ComputedStyle,
) {
    let document = context.document();
    match element_name(document, id) {
        Some("br") => state.inline.push_break(style),
        Some("img") => place_inline_image(context, state, id, style, container),
        Some("button") => {
            if let Some(kind) = control_kind_of(document, id) {
                place_control(context, state, id, kind, style, container);
            }
        }
        _ => {
            for child in document.children(id) {
                handle_node(context, state, child, container);
            }
        }
    }
}
