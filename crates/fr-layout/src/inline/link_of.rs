use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;

pub fn link_of(document: &Document, id: NodeId) -> Option<NodeId> {
    let mut current = Some(id);
    while let Some(candidate) = current {
        let node = document.node(candidate)?;
        if let Some(element) = node.kind().as_element()
            && element.name() == "a"
            && element.attribute("href").is_some()
        {
            return Some(candidate);
        }
        current = node.parent();
    }
    None
}
