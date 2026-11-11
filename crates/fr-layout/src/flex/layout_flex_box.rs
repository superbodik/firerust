use fr_css::cascade::computed_style::ComputedStyle;
use fr_dom::node::node_id::NodeId;
use fr_geom::geometry::rect::Rect;
use taffy::{AvailableSpace, LayoutOutput, Size, TaffyTree, compute_leaf_layout};

use crate::block::block_result::BlockResult;
use crate::block::block_spec::BlockSpec;
use crate::block::explicit_content_height::explicit_content_height;
use crate::boxes::box_kind::BoxKind;
use crate::boxes::layout_box::LayoutBox;
use crate::context::layout_context::LayoutContext;
use crate::flex::collect_flex_items::collect_flex_items;
use crate::flex::layout_item::layout_item;
use crate::flex::measure_item::measure_item;
use crate::flex::taffy_style::{taffy_container_style, taffy_item_style};
use crate::metrics::resolve_metrics::resolve_metrics;
use crate::metrics::resolve_radii::resolve_radii;
use crate::metrics::resolve_widths::resolve_widths;

pub fn layout_flex_box(
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
    let explicit_height = explicit_content_height(style, &metrics, spec.forced_height);

    let items = collect_flex_items(context, node, style);
    let mut tree: TaffyTree<usize> = TaffyTree::new();
    let mut leaves = Vec::with_capacity(items.len());
    for (index, item) in items.iter().enumerate() {
        if let Ok(leaf) = tree.new_leaf_with_context(taffy_item_style(item.style()), index) {
            leaves.push(leaf);
        }
    }
    let root_style = taffy_container_style(style, widths.content, explicit_height);
    let container_width = widths.content;

    let computed = tree
        .new_with_children(root_style, &leaves)
        .and_then(|root| {
            tree.compute_layout_with_measure(
                root,
                Size {
                    width: AvailableSpace::Definite(widths.content),
                    height: AvailableSpace::MaxContent,
                },
                |input, _id, item_context, taffy_style| match item_context {
                    Some(index) => compute_leaf_layout(
                        input,
                        taffy_style,
                        |_, _| 0.0,
                        |known, available| {
                            measure_item(context, &items[*index], container_width, known, available)
                        },
                    ),
                    None => LayoutOutput::from_outer_size(Size {
                        width: 0.0,
                        height: 0.0,
                    }),
                },
            )
            .map(|()| root)
        });

    let mut children: Vec<LayoutBox> = Vec::new();
    let mut content_height = explicit_height.unwrap_or(0.0);
    if let Ok(root) = computed {
        if explicit_height.is_none()
            && let Ok(root_layout) = tree.layout(root)
        {
            content_height = root_layout.size.height;
        }
        for (index, leaf) in leaves.iter().enumerate() {
            let Ok(item_layout) = tree.layout(*leaf) else {
                continue;
            };
            let rect = Rect::from_xywh(
                content_x + item_layout.location.x,
                content_y + item_layout.location.y,
                item_layout.size.width,
                item_layout.size.height,
            );
            children.push(layout_item(context, &items[index], rect, container_width));
        }
    }

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
    BlockResult {
        layout_box,
        margin_bottom: metrics.margin.bottom,
    }
}
