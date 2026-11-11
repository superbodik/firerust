use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::value::display::Display;
use fr_dom::node::node_id::NodeId;
use fr_geom::geometry::rect::Rect;

use crate::block::block_result::BlockResult;
use crate::block::block_spec::BlockSpec;
use crate::block::explicit_content_height::explicit_content_height;
use crate::block::layout_marker::layout_marker;
use crate::boxes::box_kind::BoxKind;
use crate::boxes::layout_box::LayoutBox;
use crate::context::layout_context::LayoutContext;
use crate::flow::first_block_child::first_block_child;
use crate::flow::layout_flow::layout_flow;
use crate::metrics::collapse_margins::collapse_margins;
use crate::metrics::content_area::ContentArea;
use crate::metrics::resolve_metrics::resolve_metrics;
use crate::metrics::resolve_radii::resolve_radii;
use crate::metrics::resolve_widths::resolve_widths;

pub fn layout_normal_block(
    context: &mut LayoutContext<'_>,
    node: NodeId,
    style: &ComputedStyle,
    spec: BlockSpec,
) -> BlockResult {
    let metrics = resolve_metrics(style, spec.available_width);
    let widths = resolve_widths(style, &metrics, spec.available_width, spec.forced_width);
    let border_left = spec.x + widths.margin_left;
    let content_x = border_left + metrics.border.left + metrics.padding.left;
    let content_y = spec.y + metrics.border.top + metrics.padding.top;

    let is_plain = matches!(style.display, Display::Block | Display::ListItem);
    let explicit_height = explicit_content_height(style, &metrics, spec.forced_height);
    let absorbs_top = is_plain
        && metrics.border.top == 0.0
        && metrics.padding.top == 0.0
        && first_block_child(context.document(), context.styles(), node).is_some();
    let absorbs_bottom = is_plain
        && explicit_height.is_none()
        && metrics.border.bottom == 0.0
        && metrics.padding.bottom == 0.0;

    let flow = layout_flow(
        context,
        node,
        style,
        ContentArea {
            x: content_x,
            y: content_y,
            width: widths.content,
        },
        absorbs_top,
        absorbs_bottom,
    );
    let mut children = flow.boxes;
    if style.display == Display::ListItem {
        children.insert(0, layout_marker(style, content_x, content_y));
    }

    let content_height = explicit_height.unwrap_or(flow.content_height);
    let rect = Rect::from_xywh(
        border_left,
        spec.y,
        widths.content + metrics.horizontal_extras(),
        content_height + metrics.vertical_extras(),
    );
    let mut layout_box = LayoutBox::plain(Some(node), BoxKind::Block, rect);
    layout_box.border_width = metrics.border;
    layout_box.border_color = style.border_color;
    layout_box.background = style.background_color;
    layout_box.radii = resolve_radii(style, rect.size.width, rect.size.height);
    layout_box.children = children;

    let margin_bottom = if absorbs_bottom {
        collapse_margins(metrics.margin.bottom, flow.margin_bottom_out)
    } else {
        metrics.margin.bottom
    };
    BlockResult {
        layout_box,
        margin_bottom,
    }
}
