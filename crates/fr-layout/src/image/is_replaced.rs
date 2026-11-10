use fr_dom::node::node_id::NodeId;
use fr_dom::query::element_name::element_name;
use fr_dom::tree::document::Document;

pub fn is_replaced(document: &Document, node: NodeId) -> bool {
    element_name(document, node) == Some("img")
}
