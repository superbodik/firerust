use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;

pub fn element_name(document: &Document, id: NodeId) -> Option<&str> {
    document
        .node(id)?
        .kind()
        .as_element()
        .map(|element| element.name())
}
