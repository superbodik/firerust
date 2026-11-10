use fr_dom::error::dom_error::DomError;
use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;

use crate::state::attribute_of::attribute_of;
use crate::submit::form_of::form_of;

pub fn choose_radio(document: &mut Document, radio: NodeId) -> Result<(), DomError> {
    let name = attribute_of(document, radio, "name").unwrap_or_default();
    let scope = form_of(document, radio).unwrap_or_else(|| document.root());
    let mates: Vec<NodeId> = document
        .walk(scope)
        .filter_map(|edge| match edge {
            Edge::Enter(id) if id != radio && is_radio_named(document, id, &name) => Some(id),
            _ => None,
        })
        .collect();
    if !name.is_empty() {
        for mate in mates {
            document.remove_attribute(mate, "checked")?;
        }
    }
    document.set_attribute(radio, "checked", "")
}

fn is_radio_named(document: &Document, id: NodeId, name: &str) -> bool {
    document
        .node(id)
        .and_then(|node| node.kind().as_element())
        .is_some_and(|element| {
            element.name() == "input"
                && element
                    .attribute("type")
                    .is_some_and(|kind| kind.eq_ignore_ascii_case("radio"))
                && element.attribute("name") == Some(name)
        })
}
