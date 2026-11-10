use fr_css::cascade::style_map::StyleMap;
use fr_css::value::display::Display;
use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;

pub fn first_block_child(document: &Document, styles: &StyleMap, node: NodeId) -> Option<NodeId> {
    for child in document.children(node) {
        let child_node = document.node(child)?;
        if let Some(text) = child_node.kind().as_text() {
            if text.trim().is_empty() {
                continue;
            }
            return None;
        }
        match styles.get(child).map(|style| style.display) {
            Some(Display::None) => continue,
            Some(display) if display.is_block_level() => return Some(child),
            _ => return None,
        }
    }
    None
}
