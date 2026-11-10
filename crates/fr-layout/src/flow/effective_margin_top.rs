use fr_css::value::display::Display;
use fr_dom::node::node_id::NodeId;

use crate::context::layout_context::LayoutContext;
use crate::flow::first_block_child::first_block_child;
use crate::image::is_replaced::is_replaced;
use crate::metrics::collapse_margins::collapse_margins;
use crate::metrics::resolve_metrics::resolve_metrics;

pub fn effective_margin_top(context: &LayoutContext<'_>, node: NodeId, available: f32) -> f32 {
    let document = context.document();
    let styles = context.styles();
    let Some(style) = styles.get(node) else {
        return 0.0;
    };
    let metrics = resolve_metrics(style, available);
    let own = metrics.margin.top;
    let is_plain = matches!(style.display, Display::Block | Display::ListItem);
    if !is_plain
        || metrics.padding.top != 0.0
        || metrics.border.top != 0.0
        || is_replaced(document, node)
    {
        return own;
    }
    match first_block_child(document, styles, node) {
        Some(first) => collapse_margins(own, effective_margin_top(context, first, available)),
        None => own,
    }
}
