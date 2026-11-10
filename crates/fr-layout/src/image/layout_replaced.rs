use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::value::display::Display;
use fr_css::value::length::Length;
use fr_dom::node::node_id::NodeId;
use fr_geom::geometry::rect::Rect;
use fr_text::render::text_alignment::TextAlignment;

use crate::block::block_result::BlockResult;
use crate::block::block_spec::BlockSpec;
use crate::boxes::box_kind::BoxKind;
use crate::boxes::layout_box::LayoutBox;
use crate::context::layout_context::LayoutContext;
use crate::image::attr_length::attr_length;
use crate::image::image_size::image_size;
use crate::metrics::resolve_length::resolve_length;
use crate::metrics::resolve_metrics::resolve_metrics;

pub fn layout_replaced(
    context: &mut LayoutContext<'_>,
    node: NodeId,
    style: &ComputedStyle,
    spec: BlockSpec,
    alignment: TextAlignment,
) -> BlockResult {
    let document = context.document();
    let available = spec.available_width;
    let metrics = resolve_metrics(style, available);
    let image = context.images().get(node).cloned();

    let (width, height) = match &image {
        Some(decoded) => {
            let width_hint = resolve_length(style.size.width, available).or_else(|| {
                attr_length(document, node, "width")
                    .and_then(|length| resolve_length(length, available))
            });
            let height_hint = match style.size.height {
                Length::Px(value) => Some(value),
                _ => match attr_length(document, node, "height") {
                    Some(Length::Px(value)) => Some(value),
                    _ => None,
                },
            };
            image_size(
                (decoded.width() as f32, decoded.height() as f32),
                width_hint,
                height_hint,
                resolve_length(style.size.max_width, available),
                available - metrics.margin.left - metrics.margin.right,
            )
        }
        None => (0.0, 0.0),
    };

    let remaining = available - width - metrics.margin.left - metrics.margin.right;
    let offset = if metrics.is_margin_left_auto && metrics.is_margin_right_auto {
        metrics.margin.left + remaining.max(0.0) / 2.0
    } else {
        match alignment {
            TextAlignment::Left => metrics.margin.left,
            TextAlignment::Center => metrics.margin.left + remaining.max(0.0) / 2.0,
            TextAlignment::Right => metrics.margin.left + remaining.max(0.0),
        }
    };
    let rect = Rect::from_xywh(spec.x + offset, spec.y, width, height);
    let kind = match image {
        Some(decoded) => BoxKind::Image(decoded),
        None => BoxKind::Block,
    };
    let margin_bottom = if style.display == Display::Block {
        metrics.margin.bottom
    } else {
        0.0
    };
    BlockResult {
        layout_box: LayoutBox::plain(Some(node), kind, rect),
        margin_bottom,
    }
}
