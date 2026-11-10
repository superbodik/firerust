use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::value::edges::Edges;
use fr_dom::node::node_id::NodeId;
use fr_geom::geometry::rect::Rect;

use crate::boxes::box_kind::BoxKind;
use crate::boxes::layout_box::LayoutBox;
use crate::context::layout_context::LayoutContext;
use crate::control::control_colors::{
    CONTROL_TEXT, FOCUS_BORDER, IDLE_BORDER, PLACEHOLDER_TEXT, default_background,
};
use crate::control::control_kind::ControlKind;
use crate::control::control_size::control_size;
use crate::control::control_text_box::control_text_box;
use crate::control::label_of::label_of;
use crate::control::label_width::label_width;

pub fn build_control_box(
    context: &mut LayoutContext<'_>,
    node: NodeId,
    kind: ControlKind,
    style: &ComputedStyle,
    x: f32,
    y: f32,
    available: f32,
) -> LayoutBox {
    let document = context.document();
    let label = label_of(document, node, kind, context.focus);
    let measured = label_width(context.text, &label.text, style);
    let (width, height) = control_size(document, node, kind, style, measured, available);
    let frame = Rect::from_xywh(x, y, width, height);
    let is_focused = context.focus.is_some_and(|focus| focus.node == node);
    let mut boxed = LayoutBox::plain(Some(node), BoxKind::Block, frame);
    boxed.border_width = Edges::uniform(1.0);
    boxed.border_color = if is_focused {
        FOCUS_BORDER
    } else {
        IDLE_BORDER
    };
    boxed.background = if style.background_color.a > 0 {
        style.background_color
    } else {
        default_background(kind)
    };
    let color = if label.is_placeholder {
        PLACEHOLDER_TEXT
    } else {
        CONTROL_TEXT
    };
    if let Some(text_box) =
        control_text_box(context.text, node, kind, style, &label.text, color, frame)
    {
        boxed.children.push(text_box);
    }
    boxed
}
