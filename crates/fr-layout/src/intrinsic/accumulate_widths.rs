use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::value::display::Display;
use fr_dom::node::node_id::NodeId;
use fr_dom::query::element_name::element_name;
use fr_text::render::content_widths::ContentWidths;

use crate::context::layout_context::LayoutContext;
use crate::inline::inline_buffer::InlineBuffer;
use crate::intrinsic::flush_widths::flush_widths;
use crate::intrinsic::image_widths::image_widths;
use crate::intrinsic::outer_widths::outer_widths;
use crate::intrinsic::widen::widen;

pub fn accumulate_widths(
    context: &mut LayoutContext<'_>,
    id: NodeId,
    container: &ComputedStyle,
    buffer: &mut InlineBuffer,
    widths: &mut ContentWidths,
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
        buffer.push_text(text, style);
        return;
    }
    let Some(style) = styles.get(id) else {
        return;
    };
    match style.display {
        Display::None => {}
        Display::Inline => match element_name(document, id) {
            Some("br") => buffer.push_break(style),
            Some("img") => {
                flush_widths(context, buffer, widths);
                widen(widths, image_widths(context, id, style));
            }
            _ => {
                for child in document.children(id) {
                    accumulate_widths(context, child, container, buffer, widths);
                }
            }
        },
        _ => {
            flush_widths(context, buffer, widths);
            let outer = outer_widths(context, id, style);
            widen(widths, outer);
        }
    }
}
