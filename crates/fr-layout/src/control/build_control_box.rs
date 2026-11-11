use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::value::edges::Edges;
use fr_dom::node::node_id::NodeId;
use fr_geom::color::rgba::Rgba;
use fr_geom::geometry::rect::Rect;

use crate::boxes::box_kind::BoxKind;
use crate::boxes::layout_box::LayoutBox;
use crate::context::layout_context::LayoutContext;
use crate::control::control_colors::{FOCUS_BORDER, IDLE_BORDER, default_background};
use crate::control::control_kind::ControlKind;
use crate::control::control_size::control_size;
use crate::control::control_text_box::control_text_box;
use crate::control::label_of::label_of;
use crate::control::label_width::label_width;
use crate::control::mix_colors::mix_colors;
use crate::metrics::resolve_radii::resolve_radii;

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
    let is_mark = matches!(kind, ControlKind::Checkbox | ControlKind::Radio);
    let is_checked = is_mark && !label.text.is_empty();
    let accent = style.accent_color.unwrap_or(FOCUS_BORDER);
    let has_authored_border = style.border_width != Edges::uniform(0.0);
    boxed.border_width = if has_authored_border {
        style.border_width
    } else {
        Edges::uniform(1.0)
    };
    boxed.border_color = if is_focused || is_checked {
        accent
    } else if has_authored_border {
        style.border_color
    } else {
        IDLE_BORDER
    };
    boxed.background = if is_checked {
        accent
    } else if style.background_color.a > 0 {
        style.background_color
    } else {
        default_background(kind)
    };
    boxed.radii = resolve_radii(style, width, height);
    let color = if is_checked {
        Rgba::opaque(255, 255, 255)
    } else if label.is_placeholder {
        mix_colors(style.color, boxed.background, 0.5)
    } else {
        style.color
    };
    if let Some(text_box) =
        control_text_box(context.text, node, kind, style, &label.text, color, frame)
    {
        boxed.children.push(text_box);
    }
    boxed
}
