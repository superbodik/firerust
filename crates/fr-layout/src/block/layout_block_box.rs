use fr_css::value::display::Display;
use fr_dom::node::node_id::NodeId;
use fr_geom::geometry::rect::Rect;
use fr_text::render::text_alignment::TextAlignment;

use crate::block::block_result::BlockResult;
use crate::block::block_spec::BlockSpec;
use crate::block::layout_normal_block::layout_normal_block;
use crate::boxes::box_kind::BoxKind;
use crate::boxes::layout_box::LayoutBox;
use crate::context::layout_context::LayoutContext;
use crate::flex::layout_flex_box::layout_flex_box;
use crate::image::is_replaced::is_replaced;
use crate::image::layout_replaced::layout_replaced;

pub fn layout_block_box(
    context: &mut LayoutContext<'_>,
    node: NodeId,
    spec: BlockSpec,
) -> BlockResult {
    let styles = context.styles();
    let Some(style) = styles.get(node) else {
        return empty_result(node, spec);
    };
    if is_replaced(context.document(), node) {
        return layout_replaced(context, node, style, spec, TextAlignment::Left);
    }
    if style.display == Display::Flex {
        return layout_flex_box(context, node, style, spec);
    }
    layout_normal_block(context, node, style, spec)
}

fn empty_result(node: NodeId, spec: BlockSpec) -> BlockResult {
    BlockResult {
        layout_box: LayoutBox::plain(
            Some(node),
            BoxKind::Block,
            Rect::from_xywh(spec.x, spec.y, 0.0, 0.0),
        ),
        margin_bottom: 0.0,
    }
}
