use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;

pub fn text_length(document: &Document, node: NodeId) -> usize {
    let mut total = 0;
    for edge in document.walk(node) {
        let Edge::Enter(id) = edge else {
            continue;
        };
        let Some(text) = document.node(id).and_then(|found| found.kind().as_text()) else {
            continue;
        };
        total += text.trim().chars().count();
    }
    total
}
