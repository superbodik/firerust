use fr_css::cascade::computed_style::ComputedStyle;
use fr_dom::node::node_id::NodeId;
use fr_text::render::content_widths::ContentWidths;

use crate::context::layout_context::LayoutContext;
use crate::control::control_kind::ControlKind;
use crate::control::control_size::control_size;
use crate::control::label_of::label_of;
use crate::control::label_width::label_width;

pub fn control_widths(
    context: &mut LayoutContext<'_>,
    node: NodeId,
    kind: ControlKind,
    style: &ComputedStyle,
) -> ContentWidths {
    let document = context.document();
    let label = label_of(document, node, kind, None);
    let measured = label_width(context.text, &label.text, style);
    let (width, _) = control_size(document, node, kind, style, measured, 0.0);
    ContentWidths {
        min: width,
        max: width,
    }
}
