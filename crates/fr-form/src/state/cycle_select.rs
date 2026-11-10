use fr_dom::error::dom_error::DomError;
use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;

use crate::state::attribute_of::attribute_of;

pub fn cycle_select(document: &mut Document, select: NodeId) -> Result<(), DomError> {
    let options: Vec<NodeId> = document
        .walk(select)
        .filter_map(|edge| match edge {
            Edge::Enter(id)
                if document
                    .node(id)
                    .and_then(|node| node.kind().as_element())
                    .is_some_and(|element| element.name() == "option") =>
            {
                Some(id)
            }
            _ => None,
        })
        .collect();
    if options.is_empty() {
        return Ok(());
    }
    let current = options
        .iter()
        .position(|&id| attribute_of(document, id, "selected").is_some())
        .unwrap_or(0);
    let next = (current + 1) % options.len();
    for &option in &options {
        document.remove_attribute(option, "selected")?;
    }
    document.set_attribute(options[next], "selected", "")
}
