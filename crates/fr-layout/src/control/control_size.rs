use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::value::length::Length;
use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;

use crate::control::control_kind::ControlKind;
use crate::metrics::resolve_length::resolve_length;

const CHARACTER_WIDTH_FACTOR: f32 = 0.55;
const CHECK_BOX_SIZE: f32 = 14.0;

pub fn control_size(
    document: &Document,
    node: NodeId,
    kind: ControlKind,
    style: &ComputedStyle,
    label_width: f32,
    available: f32,
) -> (f32, f32) {
    let font_size = style.font_size;
    let line = style.line_height.resolve(font_size);
    let styled_width = resolve_length(style.size.width, available);
    let styled_height = match style.size.height {
        Length::Px(value) => Some(value),
        _ => None,
    };
    let number = |name: &str, default: f32| {
        document
            .node(node)
            .and_then(|found| found.kind().as_element())
            .and_then(|element| element.attribute(name))
            .and_then(|text| text.trim().parse::<f32>().ok())
            .filter(|value| *value > 0.0)
            .unwrap_or(default)
    };
    let (width, height) = match kind {
        ControlKind::Checkbox | ControlKind::Radio => (CHECK_BOX_SIZE, CHECK_BOX_SIZE),
        ControlKind::Text | ControlKind::Password => (
            number("size", 20.0) * font_size * CHARACTER_WIDTH_FACTOR + 12.0,
            line + 8.0,
        ),
        ControlKind::TextArea => (
            number("cols", 20.0) * font_size * CHARACTER_WIDTH_FACTOR + 12.0,
            number("rows", 2.0) * line + 8.0,
        ),
        ControlKind::Select => ((label_width + 24.0).max(80.0), line + 8.0),
        ControlKind::Button => ((label_width + 24.0).max(48.0), line + 10.0),
    };
    (
        styled_width.unwrap_or(width),
        styled_height.unwrap_or(height),
    )
}
