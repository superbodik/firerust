use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;

use crate::select::previous_element::previous_element;

pub fn index_from_start(document: &Document, id: NodeId) -> i32 {
    let mut index = 1;
    let mut current = previous_element(document, id);
    while let Some(sibling) = current {
        index += 1;
        current = previous_element(document, sibling);
    }
    index
}
