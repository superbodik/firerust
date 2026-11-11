use crate::node::node_id::NodeId;
use crate::tree::document::Document;
use crate::tree::edge::Edge;

pub fn find_anchor(document: &Document, fragment: &str) -> Option<NodeId> {
    if fragment.is_empty() {
        return None;
    }
    let mut named: Option<NodeId> = None;
    for edge in document.walk(document.root()) {
        let Edge::Enter(id) = edge else {
            continue;
        };
        let Some(element) = document.node(id).and_then(|node| node.kind().as_element()) else {
            continue;
        };
        if element.attribute("id") == Some(fragment) {
            return Some(id);
        }
        if named.is_none() && element.name() == "a" && element.attribute("name") == Some(fragment) {
            named = Some(id);
        }
    }
    named
}
