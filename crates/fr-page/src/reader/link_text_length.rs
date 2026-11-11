use fr_dom::node::node_id::NodeId;
use fr_dom::query::element_name::element_name;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;

pub fn link_text_length(document: &Document, node: NodeId) -> usize {
    let mut total = 0;
    let mut depth = 0usize;
    for edge in document.walk(node) {
        match edge {
            Edge::Enter(id) => {
                if element_name(document, id) == Some("a") {
                    depth += 1;
                }
                if depth == 0 {
                    continue;
                }
                if let Some(text) = document.node(id).and_then(|found| found.kind().as_text()) {
                    total += text.trim().chars().count();
                }
            }
            Edge::Leave(id) => {
                if element_name(document, id) == Some("a") {
                    depth = depth.saturating_sub(1);
                }
            }
        }
    }
    total
}
