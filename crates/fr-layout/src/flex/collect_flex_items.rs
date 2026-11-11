use fr_css::cascade::computed_style::ComputedStyle;
use fr_css::cascade::style_map::StyleMap;
use fr_css::value::display::Display;
use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;

use crate::context::layout_context::LayoutContext;
use crate::control::control_kind_of::control_kind_of;
use crate::flex::anonymous_style::anonymous_style;
use crate::flex::flex_item::FlexItem;
use crate::image::is_replaced::is_replaced;

pub fn collect_flex_items(
    context: &LayoutContext<'_>,
    node: NodeId,
    container: &ComputedStyle,
) -> Vec<FlexItem> {
    let document = context.document();
    let styles = context.styles();
    let mut items: Vec<FlexItem> = Vec::new();
    let mut group: Vec<NodeId> = Vec::new();

    for child in document.children(node) {
        let Some(child_node) = document.node(child) else {
            continue;
        };
        if child_node.kind().as_text().is_some() {
            group.push(child);
            continue;
        }
        match styles.get(child) {
            None => {}
            Some(style) if style.display == Display::None => {}
            Some(style)
                if style.display == Display::Inline
                    && (is_replaced(document, child) || is_control(document, child)) =>
            {
                group.push(child)
            }
            Some(style) => {
                flush_group(document, styles, container, &mut group, &mut items);
                items.push(FlexItem::Element {
                    node: child,
                    style: *style,
                });
            }
        }
    }
    flush_group(document, styles, container, &mut group, &mut items);
    items
}

fn flush_group(
    document: &Document,
    styles: &StyleMap,
    container: &ComputedStyle,
    group: &mut Vec<NodeId>,
    items: &mut Vec<FlexItem>,
) {
    let nodes = std::mem::take(group);
    let has_content = nodes.iter().any(|&id| {
        let Some(node) = document.node(id) else {
            return false;
        };
        match node.kind().as_text() {
            Some(text) => !text.trim().is_empty(),
            None => styles
                .get(id)
                .is_some_and(|style| style.display != Display::None),
        }
    });
    if has_content {
        items.push(FlexItem::Anonymous {
            nodes,
            style: anonymous_style(container),
        });
    }
}

fn is_control(document: &Document, node: NodeId) -> bool {
    control_kind_of(document, node).is_some()
}
