use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;

pub fn previous_element(document: &Document, id: NodeId) -> Option<NodeId> {
    let mut current = document.node(id)?.prev_sibling();
    while let Some(candidate) = current {
        let node = document.node(candidate)?;
        if node.kind().as_element().is_some() {
            return Some(candidate);
        }
        current = node.prev_sibling();
    }
    None
}
