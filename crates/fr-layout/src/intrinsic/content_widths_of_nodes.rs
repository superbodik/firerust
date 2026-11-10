use fr_css::cascade::computed_style::ComputedStyle;
use fr_dom::node::node_id::NodeId;
use fr_text::render::content_widths::ContentWidths;

use crate::context::layout_context::LayoutContext;
use crate::inline::inline_buffer::InlineBuffer;
use crate::intrinsic::accumulate_widths::accumulate_widths;
use crate::intrinsic::flush_widths::flush_widths;

pub fn content_widths_of_nodes(
    context: &mut LayoutContext<'_>,
    nodes: &[NodeId],
    container: &ComputedStyle,
) -> ContentWidths {
    let mut buffer = InlineBuffer::new();
    let mut widths = ContentWidths { min: 0.0, max: 0.0 };
    for &node in nodes {
        accumulate_widths(context, node, container, &mut buffer, &mut widths);
    }
    flush_widths(context, &mut buffer, &mut widths);
    widths
}
