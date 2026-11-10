use crate::node::node_id::NodeId;
use crate::tree::document::Document;
use crate::tree::edge::Edge;

pub fn text_content(document: &Document, id: NodeId) -> String {
    document
        .walk(id)
        .filter_map(|edge| match edge {
            Edge::Enter(node) => document.node(node)?.kind().as_text(),
            Edge::Leave(_) => None,
        })
        .collect()
}
