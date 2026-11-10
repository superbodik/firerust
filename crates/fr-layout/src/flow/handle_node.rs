use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::value::display::Display;
use fr_dom::node::node_id::NodeId;
use fr_dom::query::element_name::element_name;

use crate::context::layout_context::LayoutContext;
use crate::flow::flow_state::FlowState;
use crate::flow::place_block::place_block;
use crate::image::place_inline_image::place_inline_image;
use crate::inline::flush_inline::flush_inline;
use crate::inline::link_of::link_of;

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
        state.inline.push_text(text, style, link_of(document, id));
        return;
    }
    let Some(style) = styles.get(id) else {
        return;
    };
    match style.display {
        Display::None => {}
        Display::Inline => handle_inline(context, state, id, style, container),
        _ => {
            flush_inline(context, state, container);
            place_block(context, state, id);
        }
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
        _ => {
            for child in document.children(id) {
                handle_node(context, state, child, container);
            }
        }
    }
}
