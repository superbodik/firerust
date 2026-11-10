use fr_css::value::display::Display;
use fr_dom::node::node_id::NodeId;
use fr_text::render::content_widths::ContentWidths;

use crate::context::layout_context::LayoutContext;
use crate::flex::flex_widths::flex_widths;
use crate::image::is_replaced::is_replaced;
use crate::intrinsic::content_widths_of_nodes::content_widths_of_nodes;
use crate::intrinsic::image_widths::image_widths;
use crate::table::table_widths::table_widths;

pub fn content_widths_of(context: &mut LayoutContext<'_>, node: NodeId) -> ContentWidths {
    let document = context.document();
    let styles = context.styles();
    let Some(style) = styles.get(node) else {
        return ContentWidths { min: 0.0, max: 0.0 };
    };
    if is_replaced(document, node) {
        return image_widths(context, node, style);
    }
    if style.display == Display::Flex {
        return flex_widths(context, node, style);
    }
    if style.display == Display::Table {
        return table_widths(context, node);
    }
    let children: Vec<NodeId> = document.children(node).collect();
    content_widths_of_nodes(context, &children, style)
}
