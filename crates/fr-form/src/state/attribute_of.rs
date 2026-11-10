use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;

pub fn attribute_of(document: &Document, node: NodeId, name: &str) -> Option<String> {
    document
        .node(node)?
        .kind()
        .as_element()?
        .attribute(name)
        .map(str::to_owned)
}
