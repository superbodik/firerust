use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;

use crate::submit::field_for_control::field_for_control;
use crate::submit::form_field::FormField;

pub fn collect_fields(
    document: &Document,
    form: NodeId,
    submitter: Option<NodeId>,
) -> Vec<FormField> {
    document
        .walk(form)
        .filter_map(|edge| match edge {
            Edge::Enter(id) if id != form => field_for_control(document, id, submitter),
            _ => None,
        })
        .collect()
}
