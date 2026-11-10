use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;

pub fn text_footprint(document: &Document) -> usize {
    document
        .walk(document.root())
        .filter_map(|edge| match edge {
            Edge::Enter(id) => document.node(id),
            Edge::Leave(_) => None,
        })
        .map(|node| match node.kind().as_text() {
            Some(text) => text.len(),
            None => 0,
        })
        .sum()
}
