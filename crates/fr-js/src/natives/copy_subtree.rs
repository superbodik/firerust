use fr_dom::error::dom_error::DomError;
use fr_dom::node::node_id::NodeId;
use fr_dom::node::node_kind::NodeKind;
use fr_dom::tree::document::Document;

pub fn copy_subtree(
    source: &Document,
    root: NodeId,
    target: &mut Document,
) -> Result<NodeId, DomError> {
    let top = copy_node(source, root, target)?;
    let mut pending = vec![(root, top)];
    while let Some((from, to)) = pending.pop() {
        for child in source.children(from) {
            let copy = copy_node(source, child, target)?;
            target.append_child(to, copy)?;
            pending.push((child, copy));
        }
    }
    Ok(top)
}

fn copy_node(source: &Document, id: NodeId, target: &mut Document) -> Result<NodeId, DomError> {
    let node = source.node(id).ok_or(DomError::UnknownNode)?;
    match node.kind() {
        NodeKind::Element(element) => {
            target.create_element(element.name(), element.attributes().to_vec())
        }
        NodeKind::Text(text) => target.create_text(text),
        NodeKind::Document => Err(DomError::UnknownNode),
    }
}
