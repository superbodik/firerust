use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;

pub fn next_element(document: &Document, id: NodeId) -> Option<NodeId> {
    let mut current = document.node(id)?.next_sibling();
    while let Some(candidate) = current {
        let node = document.node(candidate)?;
        if node.kind().as_element().is_some() {
            return Some(candidate);
        }
        current = node.next_sibling();
    }
    None
}
