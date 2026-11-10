use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;

use crate::select::attribute_matches::attribute_matches;
use crate::select::compound::Compound;

pub fn matches_compound(document: &Document, element: NodeId, compound: &Compound) -> bool {
    let Some(data) = document
        .node(element)
        .and_then(|node| node.kind().as_element())
    else {
        return false;
    };
    if let Some(tag) = &compound.tag
        && data.name() != &**tag
    {
        return false;
    }
    if let Some(id) = &compound.id
        && data.attribute("id") != Some(&**id)
    {
        return false;
    }
    compound
        .classes
        .iter()
        .all(|class| has_class(data.attribute("class"), class))
        && compound
            .attributes
            .iter()
            .all(|condition| attribute_matches(condition, data.attribute(&condition.name)))
}

fn has_class(attribute: Option<&str>, class: &str) -> bool {
    attribute.is_some_and(|value| value.split_whitespace().any(|item| item == class))
}
