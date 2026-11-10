use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;

pub fn form_of(document: &Document, control: NodeId) -> Option<NodeId> {
    let mut current = document.node(control)?.parent();
    while let Some(candidate) = current {
        let node = document.node(candidate)?;
        if node
            .kind()
            .as_element()
            .is_some_and(|element| element.name() == "form")
        {
            return Some(candidate);
        }
        current = node.parent();
    }
    None
}
