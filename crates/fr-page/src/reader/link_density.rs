use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;

use crate::reader::link_text_length::link_text_length;
use crate::reader::text_length::text_length;

pub fn link_density(document: &Document, node: NodeId) -> f32 {
    let total = text_length(document, node);
    if total == 0 {
        return 0.0;
    }
    link_text_length(document, node) as f32 / total as f32
}
