use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;

use crate::select::next_element::next_element;

pub fn index_from_end(document: &Document, id: NodeId) -> i32 {
    let mut index = 1;
    let mut current = next_element(document, id);
    while let Some(sibling) = current {
        index += 1;
        current = next_element(document, sibling);
    }
    index
}
