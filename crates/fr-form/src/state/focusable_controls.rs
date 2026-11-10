use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;
use fr_layout::control::control_kind_of::control_kind_of;

use crate::state::attribute_of::attribute_of;

pub fn focusable_controls(document: &Document) -> Vec<NodeId> {
    document
        .walk(document.root())
        .filter_map(|edge| match edge {
            Edge::Enter(id)
                if control_kind_of(document, id).is_some()
                    && attribute_of(document, id, "disabled").is_none() =>
            {
                Some(id)
            }
            _ => None,
        })
        .collect()
}
